mod handler;
mod ratelimit;
pub mod error;
pub mod model;

use std::fmt::Display;
use std::time::Duration;
use reqwest::{Client, Method, StatusCode};
use reqwest::header::HeaderMap;
use serde::Serialize;
use serde::de::DeserializeOwned;
use tokio::time::sleep;
use crate::error::{ErrorResponse, ApiError, Error};
#[cfg(feature = "members")]
use crate::model::members::Member;
#[cfg(feature = "messages")]
use crate::model::messages::ProxiedMessage;
#[cfg(feature = "systems")]
use crate::model::systems::{PublicSystemSettings, System, SystemSettings};
use crate::ratelimit::limit::{RateLimitInfo, RateLimiter};
use crate::ratelimit::scope::RateLimitScope;

pub type PKResult<T> = Result<T, Error>;
pub type PKToken<'a> = &'a str;

const API_BASE_URL: &str = "https://api.pluralkit.me/v2";
const MAX_RETRIES: u32 = 3;

pub struct PKClient {
    pub(crate) http_client: Client,
    pub(crate) rate_limit: RateLimiter,
    pub user_agent: String,
}

impl PKClient {
    pub fn new(user_agent: String) -> Self {
        Self::new_with_client(Client::new(), user_agent)
    }

    pub fn new_with_client(http_client: Client, user_agent: String) -> Self {
        Self {
            http_client,
            rate_limit: RateLimiter::new(),
            user_agent: format!("{user_agent} [pluralkit-rs {}]", env!("CARGO_PKG_VERSION")),
        }
    }

    #[cfg(feature = "systems")]
    pub async fn get_own_system(&self, token: PKToken<'_>) -> PKResult<System> {
        handler::systems::get_system(&self, Some(token), "@me").await
    }

    #[cfg(feature = "systems")]
    pub async fn get_system(&self, token: Option<PKToken<'_>>, system_ref: &str) -> PKResult<System> {
        handler::systems::get_system(&self, token, system_ref).await
    }

    #[cfg(feature = "systems")]
    pub async fn update_system(&self, token: PKToken<'_>, system: &System) -> PKResult<System> {
        handler::systems::update_system(&self, token, system).await
    }

    #[cfg(feature = "systems")]
    pub async fn get_system_settings(&self, token: PKToken<'_>, system_ref: &str) -> PKResult<SystemSettings> {
        handler::systems::get_system_settings(&self, token, system_ref).await
    }

    #[cfg(feature = "systems")]
    pub async fn get_system_settings_unauthenticated(&self, system_ref: &str) -> PKResult<PublicSystemSettings> {
        handler::systems::get_system_settings_unauthenticated(&self, system_ref).await
    }

    #[cfg(feature = "systems")]
    pub async fn update_system_settings(&self, token: PKToken<'_>, system_ref: &str, system_settings: &SystemSettings) -> PKResult<SystemSettings> {
        handler::systems::update_system_settings(&self, token, system_ref, system_settings).await
    }

    #[cfg(feature = "members")]
    pub async fn get_system_members(&self, token: Option<PKToken<'_>>, system_ref: &str) -> PKResult<Vec<Member>> {
        handler::members::get_system_members(&self, token, system_ref).await
    }

    #[cfg(feature = "members")]
    pub async fn create_member(&self, token: PKToken<'_>, member: &Member) -> PKResult<Member> {
        handler::members::create_member(&self, token, member).await
    }

    #[cfg(feature = "members")]
    pub async fn get_member(&self, token: Option<PKToken<'_>>, member_ref: &str) -> PKResult<Member> {
        handler::members::get_member(&self, token, member_ref).await
    }

    #[cfg(feature = "members")]
    pub async fn update_member(&self, token: PKToken<'_>, member: &Member) -> PKResult<Member> {
        handler::members::update_member(&self, token, member).await
    }

    #[cfg(feature = "members")]
    pub async fn delete_member(&self, token: PKToken<'_>, member_ref: &str) -> PKResult<()> {
        handler::members::delete_member(&self, token, member_ref).await
    }

    #[cfg(feature = "messages")]
    pub async fn get_proxied_message(&self, token: Option<PKToken<'_>>, message_id: &str) -> PKResult<ProxiedMessage> {
        handler::messages::get_proxied_message(&self, token, message_id).await
    }

    async fn get<U: Display, RecvBody: DeserializeOwned>(
        &self,
        token: Option<PKToken<'_>>,
        scope: RateLimitScope,
        url: U,
    ) -> PKResult<RecvBody> {
        self.request::<U, (), RecvBody>(token, scope, Method::GET, url, None, true).await.transpose().unwrap()
    }

    async fn delete<U: Display>(
        &self,
        token: PKToken<'_>,
        scope: RateLimitScope,
        url: U,
    ) -> PKResult<()> {
        self.request::<U, (), ()>(Some(token), scope, Method::DELETE, url, None, false).await.map(|_| ())
    }

    async fn request<U: Display, SendBody: Serialize + ?Sized, RecvBody: DeserializeOwned>(
        &self,
        token: Option<PKToken<'_>>,
        scope: RateLimitScope,
        method: Method,
        url: U,
        body: Option<&SendBody>,
        return_body: bool,
    ) -> PKResult<Option<RecvBody>> {
        self.rate_limit.wait_if_required(scope).await;

        let mut retries = 0;
        loop {
            let mut req = self.http_client.request(method.clone(), format!("{API_BASE_URL}{url}"))
                .header("User-Agent", &self.user_agent);
            if let Some(token) = token {
                req = req.header("Authorization", token);
            }
            if let Some(body) = body {
                req = req.json(body);
            }
            let resp = req.send().await.map_err(|err| Error::Reqwest(err))?;

            let headers = resp.headers();
            if let Some((scope, info)) = extract_rate_limit_info_from_headers(headers) {
                self.rate_limit.update(scope, info).await;
            }

            return match resp.status() {
                status if status.is_success() => {
                    if return_body {
                        Ok(Some(resp.json::<RecvBody>().await.map_err(|err| Error::Reqwest(err))?))
                    } else {
                        Ok(None)
                    }
                }
                StatusCode::TOO_MANY_REQUESTS => {
                    if let Ok(data) = resp.json::<ErrorResponse>().await {
                        if let Some(retry_after) = data.retry_after {
                            if retries < MAX_RETRIES {
                                sleep(Duration::from_millis(retry_after)).await;
                                retries += 1;
                                continue;
                            }
                        }
                    }
                    Err(Error::RateLimitExceeded)
                }
                status_code => {
                    let (error_code, message) = if let Ok(data) = resp.json::<ErrorResponse>().await {
                        (data.code, data.message)
                    } else {
                        (-1, String::new())
                    };
                    Err(Error::ApiError(ApiError {
                        status_code,
                        error_code,
                        message,
                    }))
                }
            }
        }
    }
}

fn extract_rate_limit_info_from_headers(headers: &HeaderMap) -> Option<(RateLimitScope, RateLimitInfo)> {
    let remaining = headers.get("X-RateLimit-Remaining");
    let reset_time = headers.get("X-RateLimit-Reset");
    let scope = headers.get("X-RateLimit-Scope");

    if let (Some(remaining), Some(reset_time), Some(scope)) = (remaining, reset_time, scope) {
        let remaining = remaining.to_str().ok()?.parse().ok()?;
        let reset_time = reset_time.to_str().ok()?.parse().ok()?;
        let scope = scope.to_str().ok()?.try_into().ok()?;
        return Some((scope, RateLimitInfo {
            remaining,
            reset_time,
        }));
    }
    None
}

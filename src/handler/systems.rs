use reqwest::Method;
use crate::model::systems::{PublicSystemSettings, System, SystemSettings};
use crate::{PKClient, PKResult, PKToken};
use crate::ratelimit::scope::RateLimitScope;

pub async fn get_system(client: &PKClient, token: Option<PKToken<'_>>, system_ref: &str) -> PKResult<System> {
    client.get(token, RateLimitScope::GenericGet, format!("/systems/{system_ref}")).await
}

pub async fn update_system(client: &PKClient, token: PKToken<'_>, system: &System) -> PKResult<System> {
    client.request(
        Some(token),
        RateLimitScope::GenericUpdate,
        Method::PATCH,
        format!("/systems/{}", system.uuid),
        Some(system),
        true
    ).await.transpose().unwrap()
}

pub async fn get_system_settings(client: &PKClient, token: PKToken<'_>, system_ref: &str) -> PKResult<SystemSettings> {
    client.get(Some(token), RateLimitScope::GenericGet, format!("/systems/{system_ref}/settings")).await
}

pub async fn get_system_settings_unauthenticated(client: &PKClient, system_ref: &str) -> PKResult<PublicSystemSettings> {
    client.get(None, RateLimitScope::GenericGet, format!("/systems/{system_ref}/settings")).await
}

pub async fn update_system_settings(client: &PKClient, token: PKToken<'_>, system_ref: &str, system_settings: &SystemSettings) -> PKResult<SystemSettings> {
    client.request(
        Some(token),
        RateLimitScope::GenericUpdate,
        Method::PATCH,
        format!("/systems/{system_ref}/settings"),
        Some(system_settings),
        true
    ).await.transpose().unwrap()
}
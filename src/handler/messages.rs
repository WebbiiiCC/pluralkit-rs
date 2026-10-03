use crate::{PKClient, PKResult, PKToken};
use crate::model::messages::ProxiedMessage;
use crate::ratelimit::scope::RateLimitScope;

pub async fn get_proxied_message(client: &PKClient, token: Option<PKToken<'_>>, message_id: &str) -> PKResult<ProxiedMessage> {
    client.get(token, RateLimitScope::Message, format!("/messages/{message_id}")).await
}
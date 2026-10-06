use serde::Deserialize;
use crate::model::DateTime;
use crate::model::members::Member;
use crate::model::systems::System;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct ProxiedMessage {
    pub timestamp: DateTime,
    /// The ID of the message sent by the webhook.
    pub id: String,
    /// The ID of the (now-deleted) message that triggered the proxy.
    pub original: String,
    /// The user ID of the account that triggered the proxy.
    pub sender: String,
    /// The ID of the channel the message was sent in.
    pub channel: String,
    /// The ID of the server the message was sent in.
    pub guild: String,
    /// The system that proxied the message. None if the member associated with this message was deleted.
    #[cfg(feature = "systems")]
    pub system: Option<System>,
    /// The member that proxied the message. None if the member associated with this message was deleted.
    #[cfg(feature = "members")]
    pub member: Option<Member>,
}
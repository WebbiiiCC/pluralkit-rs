use serde::{Deserialize, Serialize};
use crate::model::DateTime;
#[cfg(feature = "privacy")]
use crate::model::PrivacyValue;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Member {
    pub id: String,
    pub uuid: String,
    /// id of system this member is registered in (only set when using get_member)
    #[serde(default)]
    pub system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// 6-character hex code, no `#` at the beginning
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// `YYYY-MM-DD` format, a year value of `0004` means no year
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pronouns: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_avatar_url: Option<String>,
    #[serde(rename = "banner", skip_serializing_if = "Option::is_none")]
    pub banner_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing)]
    pub created: Option<DateTime>,
    pub proxy_tags: Vec<ProxyTag>,
    pub keep_proxy: bool,
    pub tts: bool,
    pub autoproxy_enabled: Option<bool>,
    #[serde(skip_serializing)]
    pub message_count: Option<u32>,
    #[serde(skip_serializing)]
    pub last_message_timestamp: Option<DateTime>,
    #[cfg(feature = "privacy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy: Option<MemberPrivacy>,
}

#[cfg(feature = "privacy")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MemberPrivacy {
    pub visibility: PrivacyValue,
    #[serde(rename = "name_privacy")]
    pub name: PrivacyValue,
    #[serde(rename = "description_privacy")]
    pub description: PrivacyValue,
    #[serde(rename = "birthday_privacy")]
    pub birthday: PrivacyValue,
    #[serde(rename = "pronoun_privacy")]
    pub pronouns: PrivacyValue,
    #[serde(rename = "avatar_privacy")]
    pub avatar: PrivacyValue,
    #[serde(rename = "banner_privacy")]
    pub banner: PrivacyValue,
    #[serde(rename = "metadata_privacy")]
    pub metadata: PrivacyValue,
    #[serde(rename = "proxy_privacy")]
    pub proxy: PrivacyValue,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProxyTag {
    pub prefix: Option<String>,
    pub suffix: Option<String>,
}
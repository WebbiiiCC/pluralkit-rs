use serde::{Deserialize, Serialize};
use crate::model::DateTime;
#[cfg(feature = "privacy")]
use crate::model::PrivacyValue;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct System {
    pub id: String,
    pub uuid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pronouns: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(rename = "banner", skip_serializing_if = "Option::is_none")]
    pub banner_url: Option<String>,
    /// 6-character hex code, no `#` at the beginning
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing)]
    pub created: Option<DateTime>,
    #[cfg(feature = "privacy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy: Option<SystemPrivacy>,
}

#[cfg(feature = "privacy")]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct SystemPrivacy {
    #[serde(rename = "name_privacy")]
    pub name: PrivacyValue,
    #[serde(rename = "description_privacy")]
    pub description: PrivacyValue,
    #[serde(rename = "avatar_privacy")]
    pub avatar: PrivacyValue,
    #[serde(rename = "banner_privacy")]
    pub banner: PrivacyValue,
    #[serde(rename = "pronoun_privacy")]
    pub pronouns: PrivacyValue,
    #[serde(rename = "member_list_privacy")]
    pub member_list: PrivacyValue,
    #[serde(rename = "group_list_privacy")]
    pub group_list: PrivacyValue,
    #[serde(rename = "front_privacy")]
    pub front: PrivacyValue,
    #[serde(rename = "front_history_privacy")]
    pub front_history: PrivacyValue,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct SystemSettings {
    pub timezone: String,
    /// whether proxied messages can be pinged using the 🔔 reaction
    pub pings_enabled: bool,
    /// seconds after which latch autoproxy will timeout (Default is 6 hours, 0 is "never")
    pub latch_timeout: Option<u32>,
    /// whether members created through the bot have privacy settings set to private by default
    pub member_default_private: bool,
    /// whether groups created through the bot have privacy settings set to private by default
    pub group_default_private: bool,
    /// whether the bot shows the system's own private information without a -private flag
    pub show_private_info: bool,
    #[serde(skip_serializing)]
    pub member_limit: u32,
    #[serde(skip_serializing)]
    pub group_limit: u32,
    /// whether the bot will match proxy tags matching only the case used in the trigger message
    pub case_sensitive_proxy_tags: bool,
    /// whether the bot will show errors when proxying fails
    pub proxy_error_message_enabled: bool,
    /// whether 6-character ids will be shown by the bot as two 3-character parts separated by a `-`
    pub hid_display_split: bool,
    /// whether ids will be shown by the bot in uppercase
    pub hid_display_caps: bool,
    /// whether the bot will pad 5-character ids in lists
    pub hid_list_padding: IDPaddingFormat,
    /// switch action the bot will take when proxying
    pub proxy_switch: ProxySwitchAction,
    /// format used for webhook names during proxying (defaults to `{name} {tag}`)
    pub name_format: String,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct PublicSystemSettings {
    /// whether proxied messages can be pinged using the 🔔 reaction
    pub pings_enabled: bool,
    /// seconds after which latch autoproxy will timeout (Default is 6 hours, 0 is "never")
    pub latch_timeout: Option<u32>,
    /// whether the bot will match proxy tags matching only the case used in the trigger message
    pub case_sensitive_proxy_tags: bool,
    /// whether the bot will show errors when proxying fails
    pub proxy_error_message_enabled: bool,
    /// whether 6-character ids will be shown by the bot as two 3-character parts separated by a `-`
    pub hid_display_split: bool,
    /// whether ids will be shown by the bot in uppercase
    pub hid_display_caps: bool,
    /// whether the bot will pad 5-character ids in lists
    pub hid_list_padding: IDPaddingFormat,
    /// switch action the bot will take when proxying
    pub proxy_switch: ProxySwitchAction,
    /// format used for webhook names during proxying (defaults to `{name} {tag}`)
    pub name_format: String,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum IDPaddingFormat {
    /// do not pad 5-character ids
    #[default]
    Off,
    /// add a padding space to the left of 5-character ids in lists
    Left,
    /// add a padding space to the right of 5-character ids in lists
    Right,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum ProxySwitchAction {
    /// do nothing
    #[default]
    Off,
    /// if the currently proxied member is not present in the current switch, log a new switch with this member
    New,
    /// if the current switch has 0 members, log a new switch with the currently proxied member; otherwise, add the member to the current switch
    Add,
}
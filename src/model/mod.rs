#[cfg(feature = "systems")]
pub mod systems;
#[cfg(feature = "members")]
pub mod members;
#[cfg(feature = "messages")]
pub mod messages;

#[cfg(not(feature = "chrono"))]
pub type DateTime = String;

#[cfg(feature = "chrono")]
pub type DateTime = chrono::DateTime<chrono::Utc>;

#[cfg(feature = "privacy")]
#[derive(Debug, Default, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum PrivacyValue {
    #[default]
    Public,
    Private,
}
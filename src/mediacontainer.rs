use util;

#[derive(Debug, Clone, Copy)]
#[derive(Deserialize)]
pub enum DirectoryType {
    Unknown,
    #[serde(rename = "movie")] Movies,
    #[serde(rename = "artist")] Music,
    #[serde(rename = "show")] Shows,
}

impl Default for DirectoryType {
    fn default() -> DirectoryType {
        DirectoryType::Unknown
    }
}

#[derive(Debug, Clone, Default)]
#[derive(Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub(crate) struct MediaContainer {
    identifier: String,
    size: u64,

    // Optional fields. Only one of these will generally be present in a MediaContainer, so it's up
    // to the code creating this MediaContainer to know which to expect.
    #[serde(default, rename = "Server")] pub(crate) servers: Vec<MediaServerEntry>,
    #[serde(default, rename = "Directory")] pub(crate) directories: Vec<DirectoryEntry>,
}

#[derive(Debug, Clone, Default)]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MediaServerEntry {
    pub(crate) name: String,
    pub(crate) address: String,
    pub(crate) port: u16,
    pub(crate) version: String,
    pub(crate) scheme: String,
    pub(crate) host: String,
    pub(crate) local_addresses: String,
    pub(crate) machine_identifier: String,
    pub(crate) created_at: u64,
    pub(crate) updated_at: u64,
    #[serde(deserialize_with = "util::deserialize_xml_bool")] pub(crate) owned: bool,
    #[serde(deserialize_with = "util::deserialize_xml_bool")] pub(crate) synced: bool,
    // These fields are only present on servers not owned by the current user.
    #[serde(default)] pub(crate) access_token: String,
    #[serde(default)] pub(crate) source_title: String,
    #[serde(default)] pub(crate) owner_id: u64,
    #[serde(deserialize_with = "util::deserialize_xml_bool", default)] pub(crate) home: bool,
}

#[derive(Debug, Clone, Default)]
#[derive(Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct DirectoryEntry {
    pub key: String,
    pub title: String,
    pub name: String,
    #[serde(deserialize_with = "util::deserialize_xml_bool")] pub secondary: bool,
    pub prompt: String,

    // Library section related fields
    #[serde(deserialize_with = "util::deserialize_xml_bool")] pub allow_sync: bool,
    pub art: String,
    pub composite: String,
    #[serde(deserialize_with = "util::deserialize_xml_bool")] pub filters: bool,
    #[serde(deserialize_with = "util::deserialize_xml_bool")] pub refreshing: bool,
    pub thumb: String,
    #[serde(rename = "type")] pub dir_type: DirectoryType,
    pub agent: String,
}

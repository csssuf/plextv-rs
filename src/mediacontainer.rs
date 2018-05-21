use util;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub(crate) key: String,
    pub(crate) title: String,
    pub(crate) name: String,
    #[serde(deserialize_with = "util::deserialize_xml_bool")] pub(crate) secondary: bool,
    pub(crate) prompt: String,

    // Library section related fields
    #[serde(deserialize_with = "util::deserialize_xml_bool")] pub(crate) allow_sync: bool,
    pub(crate) art: String,
    pub(crate) composite: String,
    #[serde(deserialize_with = "util::deserialize_xml_bool")] pub(crate) filters: bool,
    #[serde(deserialize_with = "util::deserialize_xml_bool")] pub(crate) refreshing: bool,
    pub(crate) thumb: String,
    #[serde(rename = "type")] pub(crate) dir_type: DirectoryType,
    pub(crate) agent: String,

    // Fields for directories within libraries
    #[serde(deserialize_with = "util::deserialize_xml_bool")] pub(crate) search: bool,
}

impl DirectoryEntry {
    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn secondary(&self) -> bool {
        self.secondary
    }

    pub fn prompt(&self) -> &str {
        &self.prompt
    }

    pub fn allow_sync(&self) -> bool {
        self.allow_sync
    }

    pub fn art(&self) -> &str {
        &self.art
    }

    pub fn composite(&self) -> &str {
        &self.composite
    }

    pub fn filters(&self) -> bool {
        self.filters
    }

    pub fn refreshing(&self) -> bool {
        self.refreshing
    }

    pub fn thumb(&self) -> &str {
        &self.thumb
    }

    pub fn dir_type(&self) -> DirectoryType {
        self.dir_type
    }

    pub fn agent(&self) -> &str {
        &self.agent
    }
}

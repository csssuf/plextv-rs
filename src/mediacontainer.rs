use util;

#[derive(Debug, Clone)]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MediaContainer {
    friendly_name: String,
    identifier: String,
    machine_identifier: String,
    size: String,

    // Optional fields. Only one of these will generally be present in a MediaContainer, so it's up
    // to the code creating this MediaContainer to know which to expect.
    #[serde(default, rename = "Server")] pub(crate) servers: Vec<MediaServerEntry>,
}

#[derive(Debug, Clone)]
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

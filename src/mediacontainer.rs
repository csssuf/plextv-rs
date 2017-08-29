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
    name: String,
    address: String,
    port: u16,
    version: String,
    scheme: String,
    host: String,
    local_addresses: String,
    machine_identifier: String,
    created_at: u64,
    updated_at: u64,
    owned: bool,
    synced: bool,
    // These fields are only present on servers not owned by the current user.
    #[serde(default)] access_token: String,
    #[serde(default)] source_title: String,
    #[serde(default)] owner_id: u64,
    #[serde(default)] home: bool,
}

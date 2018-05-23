#[derive(Debug, Clone, Default)]
#[derive(Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Media {
    pub(crate) id: u32,
    pub(crate) duration: u32,
    pub(crate) bitrate: u32,
    pub(crate) audio_channels: u32,
    pub(crate) audio_codec: String,
    pub(crate) container: String,

    #[serde(default, rename = "Part")] pub(crate) parts: Vec<Part>,
}


#[derive(Debug, Clone, Default)]
#[derive(Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Part {
    pub(crate) id: u32,
    pub(crate) key: String,
    pub(crate) duration: u32,
    pub(crate) file: String,
    pub(crate) size: u64,
    pub(crate) container: String,
}

use mediacontainer::MediaServerEntry;

#[derive(Debug, Clone)]
pub struct PlexMediaServer {
    pub(crate) entry: MediaServerEntry,
}

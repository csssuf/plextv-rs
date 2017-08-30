use mediacontainer::MediaServerEntry;

use reqwest::Client;
use reqwest::header::Headers;

#[derive(Debug, Clone)]
pub struct PlexMediaServer {
    client: Client,
    headers: Headers,
    entry: MediaServerEntry,
}

impl PlexMediaServer {
    pub(crate) fn new(
        client: Client,
        headers: Headers,
        server: MediaServerEntry,
    ) -> PlexMediaServer {
        PlexMediaServer {
            client: client,
            headers: headers,
            entry: server,
        }
    }
}

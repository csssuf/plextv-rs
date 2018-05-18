use mediacontainer::*;

use failure::Error;
use reqwest::Client;
use reqwest::header::Headers;
use serde_xml_rs;

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

    pub fn system(&self) -> Result<Vec<DirectoryEntry>, Error> {
        let res = self.client
            .get(&format!(
                "{}://{}:{}/system",
                self.entry.scheme,
                self.entry.address,
                self.entry.port
            ))?
            .headers(self.headers.clone())
            .send()?;

        let res_struct: MediaContainer = match serde_xml_rs::from_reader(res) {
            Ok(s) => s,
            Err(e) => bail!("deserialization error: {}", e),
        };

        Ok(Vec::from(res_struct.directories))
    }

    pub fn library_sections(&self) -> Result<Vec<DirectoryEntry>, Error> {
        let res = self.client
            .get(&format!(
                "{}://{}:{}/library/sections",
                self.entry.scheme,
                self.entry.address,
                self.entry.port
            ))?
            .headers(self.headers.clone())
            .send()?;

        let res_struct: MediaContainer = match serde_xml_rs::from_reader(res) {
            Ok(s) => s,
            Err(e) => bail!("deserialization error: {}", e),
        };

        Ok(Vec::from(res_struct.directories))
    }
}

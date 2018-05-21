use mediacontainer::*;

use failure::Error;
use reqwest::Client;
use serde_xml_rs;

#[derive(Debug, Clone)]
pub struct PlexMediaServer {
    client: Client,
    entry: MediaServerEntry,
}

impl PlexMediaServer {
    pub(crate) fn new(
        client: Client,
        server: MediaServerEntry,
    ) -> PlexMediaServer {
        PlexMediaServer {
            client,
            entry: server,
        }
    }

    pub fn name(&self) -> &str {
        &self.entry.name
    }

    pub fn system(&self) -> Result<Vec<DirectoryEntry>, Error> {
        let res = self.client
            .get(&format!(
                "{}://{}:{}/system",
                self.entry.scheme,
                self.entry.address,
                self.entry.port
            ))
            .send()?;

        let res_struct: MediaContainer = match serde_xml_rs::from_reader(res) {
            Ok(s) => s,
            Err(e) => bail!("deserialization error: {}", e),
        };

        Ok(res_struct.directories)
    }

    pub fn library_sections(&self) -> Result<Vec<DirectoryEntry>, Error> {
        let res = self.client
            .get(&format!(
                "{}://{}:{}/library/sections",
                self.entry.scheme,
                self.entry.address,
                self.entry.port
            ))
            .send()?;

        let res_struct: MediaContainer = match serde_xml_rs::from_reader(res) {
            Ok(s) => s,
            Err(e) => bail!("deserialization error: {}", e),
        };

        Ok(res_struct.directories)
    }
}

use library::Library;
use mediacontainer::*;

use failure::Error;
use reqwest::{Client, Response};
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

    pub(crate) fn make_request(&self, path: &str) -> Result<Response, Error> {
        self.client
            .get(&format!(
                "{}://{}:{}{}",
                self.entry.scheme,
                self.entry.address,
                self.entry.port,
                path
            ))
            .send()
            .map_err(|e| e.into())
    }

    pub fn system(&self) -> Result<Vec<DirectoryEntry>, Error> {
        let res = self.make_request("/system")?;

        let res_struct: MediaContainer = match serde_xml_rs::from_reader(res) {
            Ok(s) => s,
            Err(e) => bail!("deserialization error: {}", e),
        };

        Ok(res_struct.directories)
    }

    pub fn library_sections(&self) -> Result<Vec<Library>, Error> {
        let res = self.make_request("/library/sections")?;

        let res_struct: MediaContainer = match serde_xml_rs::from_reader(res) {
            Ok(s) => s,
            Err(e) => bail!("deserialization error: {}", e),
        };

        Ok(res_struct
           .directories
           .into_iter()
           .map(|dir| Library::new(self.clone(), dir))
           .collect())
    }
}

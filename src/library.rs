use std::ops::Deref;

use mediacontainer::{DirectoryEntry, MediaContainer};
use pms::PlexMediaServer;

use failure::Error;
use serde_xml_rs;

#[derive(Clone, Debug)]
pub struct Library<'a> {
    server: &'a PlexMediaServer,
    directory: DirectoryEntry,
}

impl<'a> Deref for Library<'a> {
    type Target = DirectoryEntry;

    fn deref(&self) -> &DirectoryEntry {
        &self.directory
    }
}

impl<'a> Library<'a> {
    pub(crate) fn new(server: &PlexMediaServer, directory: DirectoryEntry) -> Library {
        Library { server, directory }
    }

    pub fn directories(&self) -> Result<Vec<DirectoryEntry>, Error> {
        let res = self.server.make_request(&format!("/library/sections/{}", self.directory.key))?;

        let res_struct: MediaContainer = match serde_xml_rs::from_reader(res) {
            Ok(s) => s,
            Err(e) => bail!("deserialization error: {}", e),
        };

        Ok(res_struct.directories.into_iter().filter(|d| !d.search).collect())
    }
}

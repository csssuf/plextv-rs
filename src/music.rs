use media::Media;
use mediacontainer::{DirectoryEntry, MediaContainer};
use pms::PlexMediaServer;

use failure::Error;
use serde_xml_rs;

#[derive(Clone, Debug)]
pub struct MusicLibrary {
    server: PlexMediaServer,
    dirent: DirectoryEntry,
}

impl MusicLibrary {
    pub(crate) fn new(server: PlexMediaServer, dirent: DirectoryEntry) -> MusicLibrary {
        MusicLibrary {
            server,
            dirent,
        }
    }

    pub fn artists(&self) -> Result<Vec<Artist>, Error> {
        let res = self.server.make_request(&format!("/library/sections/{}/all", self.dirent.key))?;
        let res_struct: MediaContainer = match serde_xml_rs::from_reader(res) {
            Ok(s) => s,
            Err(e) => bail!("deserialization error: {}", e),
        };

        let mut out = Vec::new();
        for dirent in res_struct.directories {
            out.push(Artist {
                library: self,
                dirent,
            });
        }
        Ok(out)
    }
}

#[derive(Clone, Debug)]
pub struct Artist<'a> {
    library: &'a MusicLibrary,
    dirent: DirectoryEntry,
}

impl<'a> Artist<'a> {
    pub fn albums(&self) -> Result<Vec<Album>, Error> {
        let res = self.library.server.make_request(&self.dirent.key)?;
        let res_struct: MediaContainer = match serde_xml_rs::from_reader(res) {
            Ok(s) => s,
            Err(e) => bail!("deserialization error: {}", e),
        };

        let mut out = Vec::new();
        for dirent in res_struct.directories {
            out.push(Album {
                library: self.library,
                dirent,
            });
        }
        Ok(out)
    }

    pub fn name(&self) -> &str {
        &self.dirent.title
    }

    pub fn summary(&self) -> &str {
        &self.dirent.summary
    }
}

#[derive(Clone, Debug)]
pub struct Album<'a> {
    library: &'a MusicLibrary,
    dirent: DirectoryEntry,
}

#[derive(Debug, Clone, Default)]
#[derive(Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct ApiTrack {
    pub(crate) key: String,
    pub(crate) rating_key: String,
    #[serde(rename = "parentKey")]
    pub(crate) album_key: String,
    #[serde(rename = "parentRatingKey")]
    pub(crate) album_rating_key: String,
    #[serde(rename = "grandparentKey")]
    pub(crate) artist_key: String,
    #[serde(rename = "grandparentRatingKey")]
    pub(crate) artist_rating_key: String,
    pub(crate) title: String,
    #[serde(rename = "parentTitle")]
    pub(crate) album: String,
    #[serde(rename = "grandParentTitle")]
    pub(crate) artist: String,
    pub(crate) summary: String,
    pub(crate) index: u32,
    pub(crate) parent_index: u32,
    pub(crate) year: String,

    #[serde(default, rename = "Media")] pub(crate) media: Vec<Media>,
}


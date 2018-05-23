use mediacontainer::{DirectoryEntry, DirectoryType};
use music::MusicLibrary;
use pms::PlexMediaServer;

#[derive(Clone, Debug)]
pub enum Library {
    Movies,
    Music(MusicLibrary),
    Shows,
    Unknown,
}

impl Library {
    pub(crate) fn new(server: PlexMediaServer, directory: DirectoryEntry) -> Library {
        match directory.dir_type {
            DirectoryType::Movies => Library::Movies,
            DirectoryType::Music => Library::Music(MusicLibrary::new(server, directory)),
            DirectoryType::Shows => Library::Shows,
            _ => Library::Unknown,
        }
    }
}

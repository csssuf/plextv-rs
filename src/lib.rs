//! # plextv
//!
//! The `plextv` crate provides a Rust library for accessing both the plex.tv API and controlling
//! individual Plex Media Server instances.

extern crate chrono;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate hyper;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_xml_rs;

pub use self::library::Library;
pub use self::mediacontainer::{DirectoryEntry, DirectoryType};
pub use self::music::{MusicLibrary, ItemId, Artist, Album, Track};
pub use self::plextv::PlexTV;
pub use self::pms::PlexMediaServer;

mod library;
mod media;
mod mediacontainer;
mod music;
mod plextv;
mod pms;
mod util;

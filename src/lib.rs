//! # plextv
//!
//! The `plextv` crate provides a Rust library for accessing both the plex.tv API and controlling
//! individual Plex Media Server instances.

extern crate chrono;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate hyper;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_xml_rs;

pub use self::error::*;
pub use self::plextv::PlexTV;
pub use self::pms::PlexMediaServer;

mod error;
mod mediacontainer;
mod util;
mod plextv;
mod pms;

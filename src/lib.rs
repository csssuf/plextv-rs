//! # plextv
//!
//! The `plextv` crate provides a Rust library for accessing both the plex.tv API and controlling
//! individual Plex Media Server instances.

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate hyper;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub use self::error::*;
pub use self::plextv::PlexTV;

mod error;
mod util;
mod plextv;

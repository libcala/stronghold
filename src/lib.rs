//! Store program/save files in a unique folder (`.filestronghold-rs/`) across operating systems.
//!
//! # Getting Started
//! Add the following to your Cargo.toml:
//! 
//! ```toml
//! [dependencies]
//! stronghold = { path = "../" }
//! serde = "1.0"
//! serde_derive = "1.0"
//! ```
//!
//! This program saves a file under a folder titled with the crates name, and then opens it back up
//! again to make sure it is the same:
//!
//! ```rust
//! use stronghold::*;
//! #[macro_use]
//! extern crate serde_derive;
//! 
//! #[derive(Debug, PartialEq, Serialize, Deserialize)]
//! struct Data {
//!     x: u32,
//!     y: u32,
//!     text: String,
//! }
//! 
//! fn main() {
//!     let data: Data = Data { x: 0, y: 0, text: "Hello, world!".to_string() };
//!     let info = save!("savefile", data).unwrap();
//!     println!("Saved: {:?}", info);
//!     let file: Data = load!("savefile").unwrap();
//!     assert_eq!(data, file);
//! }
//! ```

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub use serde::{Serialize, Deserialize};
use bincode::{serialize, deserialize};

use miniz_oxide::inflate::decompress_to_vec;
use miniz_oxide::deflate::compress_to_vec;

const HEADER_V1: &'static [u8; 4] = b"St\x00\x01";

fn path(crate_name: &str, filename: &str) -> PathBuf {
    let mut path = if cfg!(not(target_os = "android")) {
        let home_dir = match std::env::var(if cfg!(target_os = "windows") {
            "HOMEPATH"
        } else {
            "HOME"
        }) {
            Ok(val) => val,
            Err(e) => panic!("couldn't interpret $HOME: {}", e),
        };

        PathBuf::from(home_dir)
    } else {
        // TODO.
        unimplemented!()
    };

    path.push(".filestronghold-rs");
    path.push(crate_name);

    std::fs::create_dir_all(&path).unwrap();

    path.push(filename);

    path
}

/// Save file information.
#[derive(Debug)]
pub struct Info {
    /// Number of bytes uncompressed.
    pub u_bytes: usize,
    /// Number of bytes compressed.
    pub c_bytes: usize,
}

/// Save a file.  Returns `None` when computer is out of space.
#[doc(hidden)]
pub fn save<T>(crate_name: &str, filename: &str, data: &T) -> Option<Info>
    where T: Serialize
{
    let mut file = if let Ok(file) = File::create(path(crate_name, filename)) {
        file
    } else {
        return None;
    };
    let data: Vec<u8> = serialize(data).unwrap();
    let compressed = compress_to_vec(&data[..], 10);

    // 4-byte Stronghold file version 0.1 (Deflate on Bincode).
    if file.write_all(HEADER_V1).is_err() {
        return None;
    }
    if file.write_all(&compressed[..]).is_err() {
        return None;
    }

    Some(Info {
        u_bytes: data.len(),
        c_bytes: compressed.len() + 4,
    })
}

/// Load a save file.  Returns `None` if it doesn't exist or is corrupted.
#[doc(hidden)]
pub fn load<T>(crate_name: &str, filename: &str) -> Option<T>
    where for<'de> T: serde::de::Deserialize<'de>
{
    let mut file = if let Ok(file) = File::open(path(crate_name, filename)) {
        file
    } else {
        return None;
    };
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data).unwrap();

    assert_eq!(&data[0..4], HEADER_V1);

    if let Ok(uncompressed) = decompress_to_vec(&data[4..]) {
        let data = deserialize(uncompressed.as_slice()).unwrap();
        Some(data)
    } else {
        None
    }
}

/// Save a file.  Returns `None` when computer is out of space.
#[macro_export] macro_rules! save {
    ($filename: expr, $data: expr) => {
        $crate::save(env!("CARGO_PKG_NAME"), $filename, &$data)
    }
}

/// Load a save file.  Returns `None` if it doesn't exist or is corrupted.
#[macro_export] macro_rules! load {
    ($filename: expr) => {
        $crate::load(env!("CARGO_PKG_NAME"), $filename)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

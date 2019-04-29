//! Store program/save files in a unique folder across operating systems.
//!
//! # Getting Started
//! Add the following to your Cargo.toml:
//!
//! ```TOML
//! [dependencies]
//! stronghold = "0.1"
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

use bincode::{deserialize, serialize};
use serde::{Deserialize, Serialize};

use zip;

fn path(filename: &str) -> PathBuf {
    let mut crate_name = PathBuf::new();
    crate_name.push(std::env::args().next().unwrap());
    let crate_name = crate_name.file_name().unwrap();

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

    path.push(".dive");
    path.push(crate_name);

    std::fs::create_dir_all(&path).unwrap();

    path.push(filename);

    path
}

/// Save a file.  Returns `true` when storage drive is out of memory.
pub fn save<T>(zipfilename: &str, filename: &str, data: &T) -> bool
where
    T: Serialize,
{
    let data: Vec<u8> = serialize(data).unwrap();
    let file = if let Ok(file) = File::create(path(zipfilename)) {
        file
    } else {
        return true;
    };
    let mut zip = zip::write::ZipWriter::new(file);
    let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated).unix_permissions(0o755);
    zip.start_file(filename, options).unwrap();
    zip.write_all(&data[..]).unwrap();

    zip.finish().unwrap();
    false
}

/// Fetch a resource from this application's resource file.
pub fn fetch<T>(filename: &str) -> Option<T>
where
    for<'de> T: Deserialize<'de>,
{
    let mut path = match std::env::current_exe() {
        Ok(exe_path) => exe_path,
        Err(_e) => return None,
    };
    path = path.with_extension("zip");

    let file = if let Ok(file) = File::open(path) {
        file
    } else {
        return None;
    };
    let mut data = Vec::<u8>::new();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    let mut file = match archive.by_name(filename) {
        Ok(file) => file,
        Err(..) => return None,
    };
    file.read_to_end(&mut data).unwrap();

    let data = deserialize(data.as_slice()).unwrap();
    Some(data)
}

/// Load a save file.  Returns `None` if it doesn't exist or is corrupted.
pub fn load<T>(zipfilename: &str, filename: &str) -> Option<T>
where
    for<'de> T: Deserialize<'de>,
{
    let file = if let Ok(file) = File::open(path(zipfilename)) {
        file
    } else {
        return None;
    };
    let mut data = Vec::<u8>::new();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    let mut file = match archive.by_name(filename) {
        Ok(file) => file,
        Err(..) => return None,
    };
    file.read_to_end(&mut data).unwrap();

    let data = deserialize(data.as_slice()).unwrap();
    Some(data)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

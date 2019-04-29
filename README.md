# Stronghold
Store program/save files in a unique folder across operating systems.

# Getting Started
Add the following to your Cargo.toml:

```toml
[dependencies]
stronghold = "0.2"
serde = "1.0"
serde_derive = "1.0"
```

This program saves a file under a folder titled with the crates name, and then opens it back up
again to make sure it is the same:

```rust
use stronghold::*;
#[macro_use]
extern crate serde_derive;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Data {
    x: u32,
    y: u32,
    text: String,
}

fn main() {
    let data: Data = Data { x: 0, y: 0, text: "Hello, world!".to_string() };
    if save("savefile.zip", "bin/data", &data) {
        panic!("Failed to save file!");
    } else {
        println!("Saved!");
    }
    let file: Data = load("savefile.zip", "bin/data").unwrap();
    assert_eq!(data, file);
    println!("Loaded successfully!");
}
```

## Features
* Load and Save user-specific files in a folder named after the crate.
* Works on Windows and Linux.
* Small file sizes using pure Rust compression (`zip`).
* Fetch ZIP'd resource files.

## Links
* [Website](https://jeronaldaron.plopgrizzly.com/stronghold)
* [Cargo](https://crates.io/crates/stronghold)
* [Documentation](https://docs.rs/stronghold)
* [Change Log](https://jeronaldaron.plopgrizzly.com/stronghold/changelog)
* [Contributors](https://jeronaldaron.plopgrizzly.com/stronghold/contributors)
* [Code of Conduct](https://jeronaldaron.plopgrizzly.com/stronghold/codeofconduct)

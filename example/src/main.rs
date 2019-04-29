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

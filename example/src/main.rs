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
    let info = save!("savefile", data).unwrap();
    println!("Saved: {:?}", info);
    let file: Data = load!("savefile").unwrap();
    assert_eq!(data, file);
}

use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct SerdeStruct {
    int: i32,
    uint: u32,
    string: String,
}

pub fn serialize_deserialize_example() {
    let point = SerdeStruct {int: 1, uint: 2, string: String::from("serde example")};
    // Convert the SerdeStruct to a JSON string
    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {serialized}", serialized=serialized);
    // Convert the JSON string back to a SerdeStruct
    let deserialized: SerdeStruct = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}

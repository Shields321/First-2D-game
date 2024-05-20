// Import the necessary trait
use prost::Message;

// Include the generated Rust code
include!(concat!(env!("OUT_DIR"), "/example.rs"));

fn main() {
    // Create a new instance of MyMessage
    let mut message = MyMessage::default();
    message.id = 123;
    message.name = "Hello".to_string();
    message.gender = "Boy".to_string();

    // Serialize the message to bytes
    let serialized_data = message.encode_to_vec();

    // Deserialize the bytes back into a message
    let deserialized_message = MyMessage::decode(&*serialized_data).unwrap();

    println!("Deserialized Message: {:?}", deserialized_message);
}

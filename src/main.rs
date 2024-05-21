// Import the necessary trait
use prost::Message;

// Include the generated Rust code
include!(concat!(env!("OUT_DIR"), "/example.rs"));

fn main() {    
    // Create a new instance of MyMessage
    let message = MyMessage {
        id: 123,
        name:"hello".to_string(),
        gender:"boy".to_string()
    };
    let player = PlayerBasicInfo{
        nickname: "shields".to_string(),
        level: 90,
        exp: 100000,
        stamina: 100,
        world_level: 7
    };
    
    // Serialize the message to bytes
    let serialized_data = message.encode_to_vec();
    let data = player.encode_to_vec();

    // Deserialize the bytes back into a message
    let deserialized_message = MyMessage::decode(&*serialized_data).unwrap();
    let data_unpacked = PlayerBasicInfo::decode(&*data).unwrap();

    println!("Deserialized Message: {:?}", deserialized_message);
    println!("Message: {:?}",data_unpacked);
}

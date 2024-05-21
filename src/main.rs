// Import the necessary trait
use prost::Message;

//this is the output directory specified in the builds.rs
include!("G:/servers/protos/protos.rs");

fn main() {    
    // Create a new instance of MyMessage
    let message = MyMessage {
        id: 123,
        name:"Shields".to_string(),
        gender:"boy".to_string(),
        playermessage: "How are you".to_string(),       
    };
    let player = PlayerBasicInfo{
        nickname: "shields".to_string(),
        level: 90,
        exp: 100000,
        stamina: 100,
        world_level: 7,
        player_message: Some(message.clone()),
        other_messages: vec![
            MyMessage{
                id: 1,
                name:"Shields".to_string(),
                gender:"Boy".to_string(),
                playermessage:"What are you doing right now".to_string()
            },
            MyMessage{
                id: 2,
                name:"Bronty".to_string(),
                gender:"Boy".to_string(),
                playermessage:"Nothing".to_string()
            },
        ]
    };
    
    // Serialize the message to bytes
    let serialized_data = message.encode_to_vec();
    let data = player.encode_to_vec();

    // Deserialize the bytes back into a message
    let deserialized_message = MyMessage::decode(&*serialized_data).unwrap();
    let data_unpacked = PlayerBasicInfo::decode(&*data).unwrap();

    println!("Deserialized Message: {:?}", deserialized_message);
    //println!("Message: {:?}",data_unpacked);

    println!("Deserialized player info\n
        Name:{0}\n
        Level:{1}\n
        Exp:{2}\n
        Stamina:{3}\n
        world_level:{4}\n
        Message:{5}",         
        data_unpacked.nickname,
        data_unpacked.level,
        data_unpacked.exp,
        data_unpacked.stamina,
        data_unpacked.world_level,
        deserialized_message.playermessage,
    );

    if let Some(first_other_message) = data_unpacked.other_messages.get(0) {        
        println!(
            "First Message Details\n\
            ID: {}\n\
            Name: {}\n\
            Gender: {}\n\
            Player Message: {}",
            first_other_message.id,
            first_other_message.name,
            first_other_message.gender,
            first_other_message.playermessage,
        );
    }else {
        println!("No other messages found.");
    }

    if let Some(second_other_message) = data_unpacked.other_messages.get(1)  {        
        println!(
            "Second Message\n\
            ID: {}\n\
            Name: {}\n\
            Gender: {}\n\
            Player Message: {}",
            second_other_message.id,
            second_other_message.name,
            second_other_message.gender,
            second_other_message.playermessage,
        )
    }else {
        println!("No other messages found.");
    }
    
}

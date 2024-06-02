use tungstenite::connect;
use tungstenite::protocol::Message;

pub fn start_client() {
    // Connect to the WebSocket server
    let (mut socket, _) = connect("ws://127.0.0.1:8080").expect("Failed to connect");
    
    loop{// Send a message to the server
        let mut line = String::new();
        let _b1 = std::io::stdin().read_line(&mut line).unwrap();
        if let Some('\n')=line.chars().next_back() {
            line.pop();
        }
        if let Some('\r')=line.chars().next_back() {
            line.pop();
        }
        
        let message = Message::Text(line.to_string());
        socket.write_message(message).expect("Failed to send message");
        
        let msg = socket.read_message().expect("Failed to read message");
        if let Message::Text(_text) = msg {
            println!("Client: Message send");
        }                
    }   
}

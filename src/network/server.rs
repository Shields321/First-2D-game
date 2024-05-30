use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};


use crate::objects::ball::player_move;


pub async fn start_server() {
    // Specify the address to bind the server to
    let addr = "127.0.0.1:8080".parse::<SocketAddr>().expect("Invalid address");

    // Create a TCP listener
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    println!("Server started. Listening on {}", addr);

    // Accept incoming connections
    loop {
        if let Ok((stream, _)) = listener.accept().await {
            // Spawn a new task to handle each connection
            tokio::spawn(handle_connection(stream));
        }
    }
}

async fn handle_connection(stream: tokio::net::TcpStream) {
    // Accept the WebSocket handshake
    let ws_stream = accept_async(stream)
        .await
        .expect("Error during WebSocket handshake");

    println!("WebSocket connection established");

    // Echo back received messages
    let (mut write, mut read) = ws_stream.split();
    // Continuously read messages from the client and echo them back
    while let Some(msg_result) = read.next().await {
        let msg = msg_result.expect("Error reading message");
        if let Message::Text(text) = msg {
            println!("Server: Received message: {}", text);
            player_move(&text);
            // Echo back the received message
            write.send(Message::Text(text.clone())).await.expect("Error sending message");            
        }
        
    }
}


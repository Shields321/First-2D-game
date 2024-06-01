use std::thread;

use game_engine::{
    core::Engine,
    types::{WindowSize, XYPair},
};
use objects::{ball::Ball, boxs::Box};
use network::{client::start_client,server::start_server};


mod network;
mod game_engine;
mod objects;

fn main() -> Result<(), anyhow::Error> {
    commands();

    let object_color = ["#8034eb","#FF0000","#FF0000FF"];//purple,red,blue
    let window_size = WindowSize {
        width: 800,
        height: 600,
    };
    let mut engine = Engine::new(&window_size)?;
    
    //create the draw info for the rectangle
    let width = 150.0;
    let length = 100.0;
    let rec_coords = XYPair{
        x: (&window_size.width/2) as f64 - length/8.0,
        y: (&window_size.width/2) as f64 - width*2.0,
    };
    //create the draw info for the second rectangle
    let width2 = 150.0;
    let length2 = 100.0;
    let rec2_coords = XYPair{
        x: (&window_size.width/2) as f64 - length,
        y: (&window_size.width/2) as f64 - width/2.0,
    };
    //create the draw info for the circle
    let radius = 24.0;
    let ball_coords = XYPair {
        x: (&window_size.width / 3) as f64 - radius,
        y: (&window_size.height / 3) as f64 - radius,
    };
    
    let ball = Ball::new(ball_coords, radius, object_color[0]);    
    let rec = Box::new(rec_coords,width,length,object_color[1]);
    let rec2 = Box::new(rec2_coords,width2,length2,object_color[2]);

    engine.add_game_object(ball);    
    engine.add_game_object(rec);
    engine.add_game_object(rec2);
    
    start_connections();    

    engine.run("Game Engine")
    
}

fn start_connections() {
    // Start the server and client in separate threads
    let _server_handle = thread::spawn(|| {
        let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
        rt.block_on(async {
            start_server().await;
        });
    });

    let _client_handle = thread::spawn(|| {
        start_client();
    });
}
fn commands(){
    print!("if you want to just use text to play the game type in 
    \n w for jump
    \n a for left
    \n d for right
    \n wa/aw for jump left
    \n wd/dw for jump right\n");
}
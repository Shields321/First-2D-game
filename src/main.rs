use std::thread;
use std::fs;

use game_engine::{
    core::Engine,
    types::{WindowSize, XYPair},
};
use objects::{ball::Ball, boxs::Box};
use network::{client::start_client,server::start_server};
use music::play_music::play;

mod network;
mod game_engine;
mod objects;
mod music;


fn main() -> Result<(), anyhow::Error> {
                      
    let songs = display_songs("src/music/songs");
    if songs.is_empty() {
        println!("No songs found in the folder.");
    } else {
        println!("Songs in the folder.\nchoose one to have no song press enter:");
        for song in &songs {
            println!("{}", song);
        }
    }

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
    
    
    let user_choice = user_unput();  
    let path = "src/music/songs/".to_owned() + &user_choice.to_owned();   
    play(&path);
    commands(); 
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
fn display_songs(file_path: &str) -> Vec<String> {
    let mut songs = Vec::new();
    if let Ok(entries) = fs::read_dir(file_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if extension == "mp3" {
                            if let Some(filename) = path.file_name() {
                                if let Some(filename_str) = filename.to_str() {
                                    songs.push(filename_str.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    songs
}
fn user_unput() -> String{
    let mut line = String::new();
        let _b1 = std::io::stdin().read_line(&mut line).unwrap();
        if let Some('\n')=line.chars().next_back() {
            line.pop();
        }
        if let Some('\r')=line.chars().next_back() {
            line.pop();
        }
    line
}
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};
use std::thread;

pub fn play(file_path: &str) {
    let file_path = file_path.to_string(); // Convert to String for ownership in the thread

    // Spawn a new thread for playing the music
    thread::spawn(move || {
        // Get an output stream handle to the default physical sound device
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        // Load the sound file in a loop
        loop {
            if let Ok(file) = File::open(&file_path) {
                let file = BufReader::new(file);
                // Decode that sound file into a source
                if let Ok(source) = Decoder::new(file) {
                    // Create a sink to control the playback
                    let sink = Sink::try_new(&stream_handle).unwrap();
                    sink.append(source);
                    sink.sleep_until_end();
                }
            }
        }
    });
}

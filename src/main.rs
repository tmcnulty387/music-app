use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::{fs::File, time::Duration};
use std::io::{self, Write};
use rodio::{Decoder, OutputStream, source::Source};
use rfd::FileDialog;


struct Song {
    id: u64,
    title: String,
    artist: String,
    file_path: String,
}

impl Song {
    fn new(title: &str, artist: &str, file_path: &str) -> Song {
        Song {
            id: Song::generate_uid(title, artist),
            title: String::from(title),
            artist: String::from(artist),
            file_path: String::from(file_path),
        }
    }

    fn generate_uid(title: &str, artist: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        title.hash(&mut hasher);
        artist.hash(&mut hasher);
        hasher.finish()
    }

    fn play(&self) {
        // Get an output stream handle to the default physical sound device
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        // Load a sound from a file, using a path relative to Cargo.toml
        let file = io::BufReader::new(File::open(&self.file_path).unwrap());
        // Decode that sound file into a source, supports Supports MP3, WAV, Ogg Vorbis and Flac.
        let source = Decoder::new(file).unwrap();

        // Get the duration of the audio file
        let mut duration = source.total_duration().unwrap_or_else(|| {
            eprintln!("Could not determine the duration of the audio file. Exiting.");
            std::process::exit(1);
        });

        // Extend the duration by 1 second to handle errors in duration calculation
        duration += Duration::from_secs(1);

        println!("Now playing {}, by {}.", &self.title, &self.artist);
        println!("Duration: {:?}", duration); //uses debug trait of Duration to print

        // Play the sound directly on the device
        stream_handle.play_raw(source.convert_samples()).expect("Stream_handle failed");

        // The sound plays in a separate audio thread,
        // so we need to keep the main thread alive while it's playing.
        std::thread::sleep(duration);
    }
}

fn handle_command(command: String) {
    let command_parts: Vec<&str> = command.trim().split_whitespace().collect();
    if command_parts.is_empty() {
        println!("No command entered.");
        return;
    }

    match command_parts[0] {
        "search" => {
            if command_parts.len() > 1 {
                let keyword = command_parts[1..].join(" ");
                search_song(Some(keyword));
            } else {
                search_song(None);
            }
        },
        "add" => {
            // if command_parts.len() < 3 {
            //     println!("Usage: add {{Title}} {{Artist}}");
            // } else {
            //     let title = command_parts[1];
            //     let artist = command_parts[2];
            //     add_song(title, artist);
            // }
            let keyphrases: Vec<&str> = command.split(' ').collect();
            
            
        },
        "remove" => {
            if command_parts.len() < 2 {
                println!("Usage: remove {{UID}}");
            } else {
                let uid = command_parts[1];
                remove_song(uid);
            }
        },
        "play" => {
            if command_parts.len() < 2 {
                println!("Usage: play {{UID}}");
            } else {
                let uid = command_parts[1];
                play_song(uid);
            }
        },
        _ => {
            println!("Unknown command: {}", command_parts[0]);
            println!("Available commands: search {{Keyword}}, add {{Title}} {{Artist}}, remove {{UID}}, play {{UID}}");
        }
    }
}

struct Playlist {
    songs: HashMap<u64, Song>,
}

impl Playlist {
    fn new() -> Playlist {
        Playlist {
            songs: HashMap::new(),
        }
    }
    
    fn search_song(keywords: Option<String>) {
        // Placeholder function for searching songs
        
    }

    fn add_song(title: &str, artist: &str) {
        // Placeholder function for adding a song
        
    }

    fn remove_song(uid: &str) {
        // Placeholder function for removing a song
        
    }

    fn play_song(uid: &str) {
        // Placeholder function for playing a song
        
    }
}

fn main() {
    let mut main = Playlist::new();
    
    print!("Welcome to music app, please enter a command: ");
    io::stdout().flush().unwrap();
    loop {
        let mut command: String = String::new();
        io::stdin()
                .read_line(&mut command)
                .expect("Failed to read line");
        println!();
        handle_command(command);
        print!("Please enter a command: ");
        io::stdout().flush().unwrap();
    }
}

// search {keywords --optional}
//      outputs a list of songs that match keywords, if no keywords outputs all songs
// add {Title} {Author}
    // opens file lookup GUI
// remove {UID}
// play {UID}
    // plays song based off UID

//
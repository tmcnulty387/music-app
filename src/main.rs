use std::{fs::File, time::Duration};
use std::io;
use rodio::{Decoder, OutputStream, source::Source};


struct Song {
    title: String,
    artist: String,
    file_path: String
}


// struct Playlist {
//     name: String,
//     songs: Vec<Song>,
// }

impl Song {
    fn new(title: &str, artist: &str, file_path: &str) -> Song {
        Song {
            title: String::from(title),
            artist: String::from(artist),
            file_path: String::from(file_path),
        }
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

// impl Playlist {
//     fn new(name: &str) -> Playlist {
//         Playlist {
//             name: String::from(name),
//             songs: Vec::new(),
//         }
//     }

//     fn add_song(&mut self, song: Song) {
//         // TODO
//     }

//     fn remove_song(&mut self, title: &str) {
//         // TODO
//     }

//     fn play_all(&self) {
//         // TODO
//     }
// }


fn main() {
    let file_path: &str = "./test.mp3";
    let song = Song::new("Test", "Someone", file_path);
    song.play();
}

// 
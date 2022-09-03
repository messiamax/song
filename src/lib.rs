#![warn(missing_debug_implementations)]

use std::{collections::HashMap, u8};

use tracks::{MidiTrack, Track};
use wave::Wave;

pub mod ctrl_f;
pub mod effects;
pub mod error;
pub mod globals;
pub mod instr;
pub mod io;
pub mod network;
pub mod time;
pub mod tracks;
pub mod utils;
pub mod wave;

pub use error::Error;

#[derive(Debug)]
pub struct Song {
    name: String,
    tracks: HashMap<u8, Track>,
}

impl Song {
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn new(name: String) -> Self {
        Self {
            name,
            tracks: HashMap::new(),
        }
    }

    pub fn add_midi_track(&mut self, mut track: MidiTrack) -> Result<(), Error> {
        for i in 0..=u8::MAX {
            if let std::collections::hash_map::Entry::Vacant(e) = self.tracks.entry(i) {
                match track.put_in_song(i) {
                    Ok(_) => (),
                    Err(err) => match err {
                        Error::Overwrite => continue,
                        _ => todo!(),
                    },
                };
                e.insert(Track::Midi(track));
                return Ok(());
            }
        }
        Err(Error::Overflow)
    }

    pub fn get_wave(&self) -> Wave {
        let mut wave = Wave::new();
        for track in self.tracks.values() {
            wave.add_consuming(track.play(), 0);
        }
        wave
    }
}

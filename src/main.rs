use song::{effects::Delay, wave::Mono};

fn main() {
    // let path = Path::new("midi_files/seven8.mid");
    // let song: song::Song<wave::Mono> = io::read_midi_file(path).unwrap();
    // let wave = song.get_wave();
    // let target = Path::new("out/hello_world.wav");
    // io::save_m_i16_wav(wave, target).unwrap();
    let instument = song::instr::Synthesizer::<Mono>::new("first".to_string());
    let delay = Delay::<Mono>::default();
}

#![allow(dead_code)]
use fantasy_in_rust::note::{Note, PitchClass};
use fantasy_in_rust::scale::{Mode, Scale, ScaleType};

fn main() {
    let scale = Scale::new(ScaleType::Diatonic, PitchClass::Fs, 4, Mode::Locrian).unwrap();
    println!("{:#?}", scale.notes())
}

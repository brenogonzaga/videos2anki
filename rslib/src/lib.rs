pub mod deck;
pub mod ffmpeg;
pub mod file;

use deck::generate_deck;
use std::path::PathBuf;

pub fn run(video_path: String) {
    let path = PathBuf::from(&video_path);
    let srt_path = path.with_extension("srt");
    let srt_path = String::from(srt_path.to_str().unwrap());
    let output_path = String::from("./output/");
    let video_name = String::from("The Kardashians S1E1");
    generate_deck(video_path, srt_path, output_path, video_name, None, None);
}

pub fn check_ffmpeg() {
    ffmpeg::install::check();
}

use std::{fs::File, io::BufReader};

use crate::ffmpeg::cut::{Audio, Video};
use crate::file;

pub fn generate_deck(
    input_video: String,
    input_srt: String,
    output_path: String,
    title: String,
    audio: Option<bool>,
    video: Option<bool>,
) {
    let audio = audio.unwrap_or(true);
    let video = video.unwrap_or(true);

    let _ = std::fs::remove_dir_all(output_path.clone());
    let _ = std::fs::create_dir_all(output_path.clone() + "/media/");
    let _ = std::fs::create_dir_all(output_path.clone() + "/csv/");

    let srt_path = File::open(input_srt).unwrap();
    let reader = BufReader::new(srt_path);
    let (sentences, times) = file::srt::sentences_and_times(reader);

    file::csv::write(&title, &times, &sentences, &output_path);

    if audio {
        println!("Cutting audio...");
        Audio::new(
            times.clone(),
            input_video.clone(),
            output_path.clone(),
            title.clone(),
        )
        .run()
        .unwrap();
    }

    if video {
        println!("Cutting video...");
        Video::new(
            times.clone(),
            input_video.clone(),
            output_path,
            title.clone(),
        )
        .run()
        .unwrap();
    }

    println!("Done!");
}

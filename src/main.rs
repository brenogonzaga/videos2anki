use std::{fs::File, io::BufReader, time::Instant};

mod ffmpeg;
mod file;
mod srt;

fn main() {
    ffmpeg::install::check();
    run();
}

pub fn run() {
    let file = File::open("./test/example.srt").unwrap();
    let reader = BufReader::new(file);

    let _ = std::fs::remove_dir_all("./output");
    let _ = std::fs::create_dir_all("./output/media/");

    let (sentences, times, count) = srt::extract::sentences_and_times(reader);

    let video_name = "The_Kardashians_S1E1".to_string();

    file::csv::write(video_name.clone(), times.clone(), sentences);

    let start = Instant::now();

    for (i, (start_time, end_time)) in times.iter().enumerate() {
        let output_name = format!(
            "{}_{}-{}",
            video_name,
            start_time.replace(":", "."),
            end_time.replace(":", ".")
        );
        let video_output_path = format!("./output/media/{}.mp4", output_name);
        let audio_output_path = format!("./output/media/{}.mp3", output_name);

        ffmpeg::cut::video(
            start_time.to_string(),
            end_time.to_string(),
            "./test/input.mkv".to_string(),
            video_output_path.clone(),
        );

        ffmpeg::cut::audio(
            start_time.to_string(),
            end_time.to_string(),
            "./test/input.mkv".to_string(),
            audio_output_path.clone(),
        );

        println!("({} of {}) done", i + 1, count,);
        let end = Instant::now();
        let duration = end.duration_since(start);
        let seconds = duration.as_secs();
        let minutes = seconds / 60;
        let seconds = seconds % 60;
        println!("Time elapsed: {}m {}s", minutes, seconds);
    }
}

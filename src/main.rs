use std::{
    fs::File,
    io::{self, BufReader},
    path::Path,
    time::Instant,
};

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
    let output_path = format!("./output/media/");
    let video_name = "The_Kardashians_S1E1".to_string();
    let input_path = input_file();

    let _ = std::fs::remove_dir_all("./output");
    let _ = std::fs::create_dir_all("./output/media/");

    let (sentences, times, count) = srt::extract::sentences_and_times(reader);
    file::csv::write(video_name.clone(), times.clone(), sentences);

    let start = Instant::now();

    for (i, (start_time, end_time)) in times.iter().enumerate() {
        let output_name = format!(
            "{}_{}-{}",
            video_name,
            start_time.replace(":", "."),
            end_time.replace(":", ".")
        );

        ffmpeg::cut::video(
            start_time.to_string(),
            end_time.to_string(),
            input_path.clone(),
            output_path.clone() + &output_name,
        );

        ffmpeg::cut::audio(
            start_time.to_string(),
            end_time.to_string(),
            input_path.clone(),
            output_path.clone() + &output_name,
        );

        calculate_time_and_remaining(start, i + 1, count);
    }
}

fn calculate_time_and_remaining(start: Instant, done: usize, missing: usize) {
    let elapsed = start.elapsed();
    let secs = elapsed.as_secs();
    let mins = secs / 60;
    let secs = secs % 60;

    println!("{} of {} done. ", done + 1, missing,);
    println!("Time elapsed: {}m {}s.", mins, secs);

    if done > 0 {
        let secs = ((secs as f64 / done as f64) * (missing - done) as f64).round() as u64;
        let mins = secs / 60;
        let secs = secs % 60;
        println!("Estimated time remaining: {}m {}s.", mins, secs);
    }
}

fn input_file() -> String {
    let mut input_path = String::new();
    loop {
        println!("Enter input file path:");
        io::stdin().read_line(&mut input_path).unwrap();
        input_path = input_path.trim().to_string();

        if Path::new(&input_path).exists() {
            break;
        } else {
            println!("File not found. Please try again.");
        }
    }
    input_path
}

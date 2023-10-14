use indicatif::{ProgressBar, ProgressStyle};
use pyo3::prelude::*;
use std::{fs::File, io::BufReader};

mod ffmpeg;
mod file;
mod srt;

/// A Python module implemented in Rust.
#[pymodule]
fn sync_scribe(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run, m)?)?;
    Ok(())
}

#[pyfunction]
fn run(input_video: String) -> PyResult<()> {
    ffmpeg::install::check();
    let file = File::open("./test/example.srt").unwrap();
    let reader = BufReader::new(file);
    let output_path = format!("./output/media/");
    let video_name = "The_Kardashians_S1E1".to_string();

    let _ = std::fs::remove_dir_all("./output");
    let _ = std::fs::create_dir_all("./output/media/");

    let (sentences, times, count) = srt::extract::sentences_and_times(reader);
    file::csv::write(video_name.clone(), times.clone(), sentences);

    let pb = ProgressBar::new(count as u64);
    let style = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%)")
        .unwrap();
    pb.set_style(style.progress_chars("#>-"));

    for (start_time, end_time) in times.iter() {
        let output_name = format!(
            "{}_{}-{}",
            video_name,
            start_time.replace(":", "."),
            end_time.replace(":", ".")
        );

        ffmpeg::cut::video(
            start_time.to_string(),
            end_time.to_string(),
            input_video.clone(),
            output_path.clone() + &output_name,
        );

        ffmpeg::cut::audio(
            start_time.to_string(),
            end_time.to_string(),
            input_video.clone(),
            output_path.clone() + &output_name,
        );

        pb.inc(1);
    }

    pb.finish_with_message("Done!");

    Ok(())
}

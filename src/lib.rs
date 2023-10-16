pub mod deck;
pub mod ffmpeg;
pub mod file;

use deck::generate_deck;
use pyo3::prelude::*;
use std::path::PathBuf;

/// A Python module implemented in Rust.
#[pymodule]
fn videos2anki(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run, m)?)?;
    m.add_function(wrap_pyfunction!(check_ffmpeg, m)?)?;
    Ok(())
}

#[pyfunction]
fn run(video_path: String) -> PyResult<()> {
    let path = PathBuf::from(&video_path);
    let srt_path = path.with_extension("srt");
    let srt_path = String::from(srt_path.to_str().unwrap());
    let output_path = String::from("./output/");
    let video_name = String::from("The Kardashians S1E1");
    generate_deck(video_path, srt_path, output_path, video_name, None, None);
    Ok(())
}

#[pyfunction]
fn check_ffmpeg() -> PyResult<()> {
    ffmpeg::install::check();
    Ok(())
}

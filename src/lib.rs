use std::{fs::File, io::BufReader};

use ffmpeg::cut::{Audio, Video};
use pyo3::prelude::*;

#[pymodule]
fn bridge(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(check_ffmpeg, m)?)?;
    m.add_function(wrap_pyfunction!(generate_audio, m)?)?;
    m.add_function(wrap_pyfunction!(generate_video, m)?)?;
    m.add_function(wrap_pyfunction!(generate_csv, m)?)?;
    Ok(())
}

#[pyfunction]
fn check_ffmpeg() -> PyResult<()> {
    ffmpeg::install::check();
    Ok(())
}

#[pyfunction]
fn generate_audio(
    py: Python,
    input_video: String,
    input_srt: String,
    output_path: String,
    title: String,
    progress: PyObject,
) -> PyResult<()> {
    let (_, times) = generate_times(input_srt);
    let callback: Box<dyn Fn(u64, u64)> = Box::new(move |done, missing| {
        let done_obj = done.into_py(py);
        let missing_obj = missing.into_py(py);
        let _result = progress.call(py, (done_obj, missing_obj), None);
    });
    Audio::new(times, input_video, output_path, title).run(&callback)?;
    Ok(())
}

#[pyfunction]
fn generate_video(
    py: Python,
    input_video: String,
    input_srt: String,
    output_path: String,
    title: String,
    progress: PyObject,
) -> PyResult<()> {
    let (_, times) = generate_times(input_srt);
    let callback: Box<dyn Fn(u64, u64)> = Box::new(move |done, missing| {
        let done_obj = done.into_py(py);
        let missing_obj = missing.into_py(py);
        let _result = progress.call(py, (done_obj, missing_obj), None);
    });
    Video::new(times, input_video, output_path, title).run(&callback)?;
    Ok(())
}

#[pyfunction]
fn generate_csv(input_srt: String, output_path: String, title: String) -> PyResult<()> {
    let (sentences, times) = generate_times(input_srt);
    ffmpeg::csv::write(&title, &times, &sentences, &output_path);
    Ok(())
}

fn generate_times(input_srt: String) -> (Vec<String>, Vec<(String, String)>) {
    let srt_path = File::open(input_srt).unwrap();
    let reader = BufReader::new(srt_path);
    let (sentences, times) = ffmpeg::srt::sentences_and_times(reader);
    (sentences, times)
}

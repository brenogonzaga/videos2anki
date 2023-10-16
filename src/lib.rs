use pyo3::prelude::*;

#[pymodule]
fn bridge(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run, m)?)?;
    m.add_function(wrap_pyfunction!(check_ffmpeg, m)?)?;
    Ok(())
}

#[pyfunction]
fn run(video_path: String) -> PyResult<()> {
    ffmpeg::run(video_path);
    Ok(())
}

#[pyfunction]
fn check_ffmpeg() -> PyResult<()> {
    ffmpeg::check_ffmpeg();
    Ok(())
}

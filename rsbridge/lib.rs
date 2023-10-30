use std::{fs::File, io::BufReader};

use ffmpeg::cut::{Audio, Video};
use pyo3::{prelude::*, IntoPy};

#[pymodule]
fn bridge(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ProcessAudio>()?;
    m.add_class::<ProcessVideo>()?;
    m.add_class::<ProcessCsv>()?;
    Ok(())
}

#[pyclass]
struct ProcessAudio {
    input_video: String,
    input_srt: String,
    output_path: String,
    title: String,
}

#[pymethods]
impl ProcessAudio {
    #[new]
    fn new(input_video: String, input_srt: String, output_path: String, title: String) -> Self {
        ProcessAudio {
            input_video,
            input_srt,
            output_path,
            title,
        }
    }

    fn run(&self, progress: PyObject) -> PyResult<()> {
        let (_, times) = generate_times(self.input_srt.clone());
        let callback: Box<dyn Fn(u64, u64)> = Box::new(move |done, missing| {
            Python::with_gil(|py| {
                let done_obj = done.into_py(py);
                let missing_obj = missing.into_py(py);
                let _result = progress.call(py, (done_obj, missing_obj), None);
            });
        });
        Audio::new(
            times,
            self.input_video.clone(),
            self.output_path.clone(),
            self.title.clone(),
        )
        .run(&callback)?;
        Ok(())
    }
}

#[pyclass]
struct ProcessVideo {
    input_video: String,
    input_srt: String,
    output_path: String,
    title: String,
}

#[pymethods]
impl ProcessVideo {
    #[new]
    fn new(input_video: String, input_srt: String, output_path: String, title: String) -> Self {
        ProcessVideo {
            input_video,
            input_srt,
            output_path,
            title,
        }
    }

    fn run(&self, progress: PyObject) -> PyResult<()> {
        let (_, times) = generate_times(self.input_srt.clone());
        let callback: Box<dyn Fn(u64, u64)> = Box::new(move |done, missing| {
            Python::with_gil(|py| {
                let done_obj = done.into_py(py);
                let missing_obj = missing.into_py(py);
                let _result = progress.call(py, (done_obj, missing_obj), None);
            });
        });
        Video::new(
            times,
            self.input_video.clone(),
            self.output_path.clone(),
            self.title.clone(),
        )
        .run(&callback)?;
        Ok(())
    }
}

#[pyclass]
struct ProcessCsv {
    input_srt: String,
    output_path: String,
    title: String,
}

#[pymethods]
impl ProcessCsv {
    #[new]
    fn new(input_srt: String, output_path: String, title: String) -> Self {
        ProcessCsv {
            input_srt,
            output_path,
            title,
        }
    }

    fn run(&self) -> PyResult<()> {
        let (sentences, times) = generate_times(self.input_srt.clone());
        file::csv::write(&self.title, &times, &sentences, &self.output_path);
        Ok(())
    }
}

fn generate_times(input_srt: String) -> (Vec<String>, Vec<(String, String)>) {
    let srt_path = File::open(input_srt).unwrap();
    let reader = BufReader::new(srt_path);
    let (sentences, times) = file::srt::sentences_and_times(reader);
    (sentences, times)
}

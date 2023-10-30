mod generate;

use ffmpeg::cut::{Audio, Video};
use file::csv::WriteCsv;
use generate::times;
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

    fn start(&self, progress: PyObject) -> PyResult<()> {
        let (_, times) = times(self.input_srt.clone());
        let callback: Box<dyn Fn(u64, u64)> = Box::new(move |done, missing| {
            Python::with_gil(|py| {
                let type_obj = "Audio";
                let done_obj = done.into_py(py);
                let missing_obj = missing.into_py(py);
                let _result = progress.call(py, (done_obj, missing_obj, type_obj), None);
            });
        });
        Audio::new(
            times,
            self.input_video.clone(),
            self.output_path.clone(),
            self.title.clone(),
        )
        .start(&callback)?;
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

    fn start(&self, progress: PyObject) -> PyResult<()> {
        let (_, times) = times(self.input_srt.clone());
        let callback: Box<dyn Fn(u64, u64)> = Box::new(move |done, missing| {
            Python::with_gil(|py| {
                let type_obj = "Video";
                let done_obj = done.into_py(py);
                let missing_obj = missing.into_py(py);
                let _result = progress.call(py, (done_obj, missing_obj, type_obj), None);
            });
        });
        Video::new(
            times,
            self.input_video.clone(),
            self.output_path.clone(),
            self.title.clone(),
        )
        .start(&callback)?;
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

    fn start(&self) -> PyResult<()> {
        let (sentences, times) = times(self.input_srt.clone());
        WriteCsv::new(
            self.title.clone(),
            times,
            sentences,
            self.output_path.clone(),
        )
        .write();
        Ok(())
    }
}

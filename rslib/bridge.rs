use ffmpeg::cut::Media;
use file::{csv::WriteCsv, srt};
use pyo3::{prelude::*, IntoPy};
use std::{
    fs::File,
    io::{BufReader, Error},
};

#[pymodule]
fn bridge(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<GenerateMedia>()?;
    Ok(())
}

#[pyclass]
struct GenerateMedia {
    input_video: String,
    input_srt: String,
    output_path: String,
    title: String,
}

#[pymethods]
impl GenerateMedia {
    #[new]
    fn new(input_video: String, input_srt: String, output_path: String, title: String) -> Self {
        GenerateMedia {
            input_video,
            input_srt,
            output_path,
            title,
        }
    }

    fn audio(&self, progress: PyObject) -> PyResult<()> {
        let (_, times) = times(self.input_srt.clone()).expect(
            "Error while parsing the srt file. Please make sure it is in the correct format.",
        );
        let callback: Box<dyn Fn(u64, u64)> = Box::new(move |done, missing| {
            Python::with_gil(|py| {
                let type_obj = "Audio";
                let done_obj = done.into_py(py);
                let missing_obj = missing.into_py(py);
                let _result = progress.call(py, (done_obj, missing_obj, type_obj), None);
            });
        });

        Media::new(
            times,
            self.input_video.clone(),
            self.output_path.clone(),
            self.title.clone(),
        )
        .extract_audio(&callback)?;
        Ok(())
    }

    fn video(&self, progress: PyObject) -> PyResult<()> {
        let (_, times) = times(self.input_srt.clone()).expect(
            "Error while parsing the srt file. Please make sure it is in the correct format.",
        );
        let callback: Box<dyn Fn(u64, u64)> = Box::new(move |done, missing| {
            Python::with_gil(|py| {
                let type_obj = "Video";
                let done_obj = done.into_py(py);
                let missing_obj = missing.into_py(py);
                let _result = progress.call(py, (done_obj, missing_obj, type_obj), None);
            });
        });

        Media::new(
            times,
            self.input_video.clone(),
            self.output_path.clone(),
            self.title.clone(),
        )
        .extract_video(&callback)?;
        Ok(())
    }

    fn csv(&self) -> PyResult<()> {
        let (sentences, times) = times(self.input_srt.clone()).expect(
            "Error while parsing the srt file. Please make sure it is in the correct format.",
        );
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

type Times = Vec<(String, String)>;
type Sentences = Vec<String>;

pub fn times(input_srt: String) -> Result<(Sentences, Times), Error> {
    let reader = BufReader::new(File::open(input_srt)?);
    let (sentences, times) = srt::sentences_and_times(reader);
    Ok((sentences, times))
}

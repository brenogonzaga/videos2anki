use std::fs::OpenOptions;
use std::io::Write;

pub struct WriteCsv<T: Into<String> + Clone> {
    file_name: T,
    times: Vec<(T, T)>,
    sentences: Vec<T>,
    output_path: T,
}

impl<T: Into<String> + Clone> WriteCsv<T> {
    pub fn new(file_name: T, times: Vec<(T, T)>, sentences: Vec<T>, output_path: T) -> Self {
        WriteCsv {
            file_name,
            times,
            sentences,
            output_path,
        }
    }

    pub fn write(&self) {
        let _ = std::fs::create_dir_all(format!("{}/csv", self.output_path.clone().into()));
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(format!(
                "{}/csv/{}.csv",
                self.output_path.clone().into(),
                self.file_name.clone().into()
            ))
            .unwrap();

        for (i, (start_time, end_time)) in self.times.iter().enumerate() {
            let name = (self.file_name.clone().into()
                + &start_time.clone().into()
                + "-"
                + &end_time.clone().into())
                .replace(' ', "_")
                .replace(':', ".");
            let video = format!("[sound:{}.mp4]", name);
            let audio = format!("[sound:{}.mp3]", name);
            let line = format!("{}\t{}\t{}\t", self.file_name.clone().into(), video, audio);
            file.write_all(line.as_bytes()).unwrap();

            let line = format!("\t{}\n", self.sentences[i + 1].clone().into());
            file.write_all(line.as_bytes()).unwrap();
        }
    }
}

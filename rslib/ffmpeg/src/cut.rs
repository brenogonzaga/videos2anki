use crate::run::run_ffmpeg_command;
use std::io::{Error, ErrorKind};

pub struct Audio<T: Into<String> + Clone> {
    times: Vec<(T, T)>,
    input_path: T,
    output_path: T,
    output_name: T,
}
pub struct Video<T: Into<String> + Clone> {
    times: Vec<(T, T)>,
    input_path: T,
    output_path: T,
    output_name: T,
}

impl<T: Into<String> + Clone> Audio<T> {
    pub fn new(times: Vec<(T, T)>, input_path: T, output_path: T, output_name: T) -> Self {
        Self {
            times,
            input_path,
            output_path,
            output_name,
        }
    }

    pub fn start(&self, progress: &dyn Fn(u64, u64)) -> Result<(), Error> {
        for (i, (start, end)) in self.times.iter().enumerate() {
            let file_name = format!(
                "{}/{}{}-{}",
                self.output_path.clone().into(),
                self.output_name.clone().into(),
                start.clone().into(),
                end.clone().into()
            );

            let file_name = file_name.replace(':', ".").replace(' ', "_");
            let commands = format!(
                "-y -ss {} -to {} {}.mp3 -vn -acodec libmp3lame -strict -2 -loglevel quiet -map 0:a:0 -af \"volume=1.5\" -af \"afade=t=out:st=4:d=1\"",
                start.clone().into(), end.clone().into(), file_name
            );

            if let Err(e) = run_ffmpeg_command(commands, self.input_path.clone().into()) {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("Failed to run ffmpeg command: {}", e),
                ));
            }

            progress(
                (i + 1).try_into().unwrap(),
                (self.times.len() - i - 1) as u64,
            );
        }
        Ok(())
    }
}

impl<T: Into<String> + Clone> Video<T> {
    pub fn new(times: Vec<(T, T)>, input_path: T, output_path: T, output_name: T) -> Self {
        Self {
            times,
            input_path,
            output_path,
            output_name,
        }
    }

    pub fn start(&self, progress: &dyn Fn(u64, u64)) -> Result<(), Error> {
        for (i, (start, end)) in self.times.iter().enumerate() {
            let file_name = format!(
                "{}/{}{}-{}",
                self.output_path.clone().into(),
                self.output_name.clone().into(),
                start.clone().into(),
                end.clone().into()
            );

            let file_name = file_name.replace(':', ".").replace(' ', "_");
            let commands = format!(
                "-y -ss {} -to {} {}.mp4 -c:v libx264 -strict -2 -loglevel quiet -map 0:v:0 -map 0:a:0 -c:a aac -ac 2 -vf \"scale=1280:720\" -crf 18 -af \"volume=1.5\" -af \"afade=t=out:st=4:d=1\"",
                start.clone().into(), end.clone().into(), file_name
            );

            if let Err(e) = run_ffmpeg_command(commands, self.input_path.clone().into()) {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("Failed to run ffmpeg command: {}", e),
                ));
            }

            progress(
                (i + 1).try_into().unwrap(),
                (self.times.len() - i - 1) as u64,
            );
        }
        Ok(())
    }
}

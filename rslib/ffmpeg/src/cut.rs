use crate::run::run_ffmpeg_command;
use std::io::{Error, ErrorKind};

pub struct Video<T: Into<String> + Clone> {
    times: Vec<(T, T)>,
    input_path: T,
    output_path: T,
    output_name: T,
}

pub trait MediaCutter<T: Into<String> + Clone> {
    fn new(times: Vec<(T, T)>, input_path: T, output_path: T, output_name: T) -> Self;
    fn extract_audio(&self, progress: &dyn Fn(u64, u64)) -> Result<(), Error>;
    fn extract_video(&self, progress: &dyn Fn(u64, u64)) -> Result<(), Error>;
}

impl<T: Into<String> + Clone> MediaCutter<T> for Video<T> {
    fn new(times: Vec<(T, T)>, input_path: T, output_path: T, output_name: T) -> Self {
        Video {
            times,
            input_path,
            output_path,
            output_name,
        }
    }

    fn extract_audio(&self, progress: &dyn Fn(u64, u64)) -> Result<(), Error> {
        extract_media(&self, "mp3", progress)
    }

    fn extract_video(&self, progress: &dyn Fn(u64, u64)) -> Result<(), Error> {
        extract_media(&self, "mp4", progress)
    }
}

fn extract_media<T: Into<String> + Clone>(
    media: &Video<T>,
    format: &str,
    progress: &dyn Fn(u64, u64),
) -> Result<(), Error> {
    for (i, (start, end)) in media.times.iter().enumerate() {
        let file_name = format!(
            "{}/{}{}-{}",
            media.output_path.clone().into(),
            media.output_name.clone().into(),
            start.clone().into(),
            end.clone().into()
        )
        .replace(':', ".")
        .replace(' ', "_");

        let commands = format!(
            "-y -ss {} -to {} {}.{} -vn -acodec libmp3lame -strict -2 -loglevel quiet -map 0:a:0 -af \"volume=1.5\" -af \"afade=t=out:st=4:d=1\"",
            start.clone().into(), end.clone().into(), file_name, format
        );

        run_ffmpeg_command(commands, media.input_path.clone().into()).map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("Failed to run ffmpeg command: {}", e),
            )
        })?;

        progress(
            (i + 1).try_into().unwrap(),
            (media.times.len() - i - 1) as u64,
        );
    }
    Ok(())
}

use crate::run::run_ffmpeg_command;
use std::io::{Error, ErrorKind};

pub struct Video;

pub struct Audio;

pub struct Media<T: Into<String> + Clone, State> {
    times: Vec<(T, T)>,
    input_path: T,
    output_path: T,
    output_name: T,
    state: std::marker::PhantomData<State>,
}

impl<T: Into<String> + Clone, State> Media<T, State> {
    pub fn new(times: Vec<(T, T)>, input_path: T, output_path: T, output_name: T) -> Self {
        Media {
            times,
            input_path,
            output_path,
            output_name,
            state: std::marker::PhantomData,
        }
    }
}

impl<T: Into<String> + Clone> Media<T, Video> {
    pub fn extract_video(&self, progress: &dyn Fn(u64, u64)) -> Result<(), Error> {
        extract_media(self, "mp4", progress)
    }
}

impl<T: Into<String> + Clone> Media<T, Audio> {
    pub fn extract_audio(&self, progress: &dyn Fn(u64, u64)) -> Result<(), Error> {
        extract_media(self, "mp3", progress)
    }
}

fn extract_media<T: Into<String> + Clone, State>(
    media: &Media<T, State>,
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

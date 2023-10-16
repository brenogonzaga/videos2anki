use crate::ffmpeg::run::run_ffmpeg_command;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::Error;

pub struct Audio {
    times: Vec<(String, String)>,
    input_file: String,
    output_path: String,
    output_name: String,
}
pub struct Video {
    times: Vec<(String, String)>,
    input_file: String,
    output_path: String,
    output_name: String,
}

impl Audio {
    pub fn new(
        times: Vec<(String, String)>,
        input_file: String,
        output_path: String,
        output_file: String,
    ) -> Self {
        Self {
            times,
            input_file,
            output_path,
            output_name: output_file,
        }
    }

    pub fn run(&self) -> Result<(), Error> {
        let pb = ProgressBar::new(self.times.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%)")
                .unwrap()
                .progress_chars("#>-"),
        );
        for (i, (start, end)) in self.times.iter().enumerate() {
            let file_name = format!(
                "{}/media/{}{}-{}",
                self.output_path, self.output_name, start, end
            );
            let file_name = file_name.replace(':', ".").replace(' ', "_");
            let commands = format!(
                "-ss {} -to {} -i {} {}.mp3 -vn -acodec libmp3lame -strict -2 -loglevel quiet -map 0:a:0 -af \"volume=1.5\" -af \"afade=t=out:st=4:d=1\"",
                start, end, self.input_file, file_name
            );
            pb.set_position((i + 1) as u64);

            let _ = run_ffmpeg_command(commands);
        }
        pb.finish_with_message("Finished processing audio");
        Ok(())
    }
}

impl Video {
    pub fn new(
        times: Vec<(String, String)>,
        input_file: String,
        output_path: String,
        output_name: String,
    ) -> Self {
        Self {
            times,
            input_file,
            output_path,
            output_name,
        }
    }

    pub fn run(&self) -> Result<(), Error> {
        let pb = ProgressBar::new(self.times.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%)")
                .unwrap()
                .progress_chars("#>-"),
        );

        for (i, (start, end)) in self.times.iter().enumerate() {
            let file_name = format!(
                "{}/media/{}{}-{}",
                self.output_path, self.output_name, start, end
            );
            let file_name = file_name.replace(':', ".").replace(' ', "_");
            let commands = format!(
                "-ss {} -to {} -i {} {}.mp4 -c:v libx264 -strict -2 -loglevel quiet -map 0:v:0 -map 0:a:0 -c:a aac -ac 2 -vf \"scale=1280:720\" -crf 18 -af \"volume=1.5\" -af \"afade=t=out:st=4:d=1\"",
                start, end, self.input_file, file_name
            );
            let _ = run_ffmpeg_command(commands);

            pb.set_position((i + 1) as u64);
        }
        pb.finish_with_message("Finished processing videos");
        Ok(())
    }
}

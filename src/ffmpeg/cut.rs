use std::process::{Command, Stdio};

pub fn video(start_time: String, end_time: String, input_file: String, output_file: String) {
    let duration = tsv_time_to_seconds(&end_time) - tsv_time_to_seconds(&start_time);
    let commands = format!(
            "-ss {} -t {} -i {} {}.mp4 -c:v libx264 -strict -2 -loglevel quiet -map 0:v:0 -map 0:a:0 -c:a aac -ac 2 -vf \"scale=1280:720\" -crf 18 -af \"volume=1.5\" -af \"afade=t=out:st=4:d=1\"",
            start_time, duration, input_file, output_file
        );
    let mut ffmpeg_video = Command::new("ffmpeg")
        .args(commands.split(' '))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    match ffmpeg_video.wait() {
        Ok(_) => print!(""),
        Err(_) => println!("Video cut unsuccessfully"),
    }
}

pub fn audio(start_time: String, end_time: String, input_file: String, output_file: String) {
    let duration = tsv_time_to_seconds(&end_time) - tsv_time_to_seconds(&start_time);
    let commands = format!(
            "-ss {} -t {} -i {} {}.mp3 -vn -acodec libmp3lame -strict -2 -loglevel quiet -map 0:a:0 -af \"volume=1.5\" -af \"afade=t=out:st=4:d=1\"",
            start_time, duration, input_file, output_file
        );
    let mut ffmpeg_audio = Command::new("ffmpeg")
        .args(commands.split(' '))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    match ffmpeg_audio.wait() {
        Ok(_) => print!(""),
        Err(_) => println!("Audio cut unsuccessfully"),
    }
}

fn tsv_time_to_seconds(time_str: &str) -> f32 {
    let time = time_str.replace('.', ":");
    let time = time.split(':').collect::<Vec<&str>>();
    let hours = time[0].parse::<f32>().unwrap();
    let minutes = time[1].parse::<f32>().unwrap();
    let seconds = time[2].parse::<f32>().unwrap();
    let milliseconds = time[3].parse::<f32>().unwrap();
    hours * 3600.0 + minutes * 60.0 + seconds + milliseconds / 1000.0
}

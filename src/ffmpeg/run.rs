use std::{
    io::Error,
    process::{Command, Stdio},
};

pub fn run_ffmpeg_command(commands: String) -> Result<(), Error> {
    let mut ffmpeg_process = Command::new("ffmpeg")
        .args(commands.split(' '))
        .stdout(Stdio::piped())
        .spawn()?;

    ffmpeg_process.wait()?;
    Ok(())
}

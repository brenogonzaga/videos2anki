use std::{
    io::Error,
    process::{Command, Stdio},
};

pub fn run_ffmpeg_command(commands: String) -> Result<(), Error> {
    #[cfg(target_os = "windows")]
    use std::os::windows::process::CommandExt;
    let mut ffmpeg_process = Command::new("ffmpeg")
        .args(commands.split(' '))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .spawn()?;

    #[cfg(not(target_os = "windows"))]
    let mut ffmpeg_process = Command::new("ffmpeg")
        .args(commands.split(' '))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .before_exec(|| {
            // This code will only run on non-Windows platforms
            Ok(())
        })
        .spawn()?;

    ffmpeg_process.wait()?;
    Ok(())
}

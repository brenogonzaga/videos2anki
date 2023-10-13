pub mod install {
    use std::process::Command;
    use which::which;

    pub fn check() {
        match which("ffmpeg") {
            Ok(_) => println!("ffmpeg is installed on this system"),
            Err(_) => {
                let system = std::env::consts::OS;
                let ffmpeg_install = match system {
                    "windows" => Command::new("powershell")
                        .arg("winget")
                        .arg("install")
                        .arg("ffmpeg")
                        .status(),
                    "linux" => Command::new("sudo")
                        .arg("apt")
                        .arg("install")
                        .arg("ffmpeg")
                        .status(),
                    _ => panic!("Unsupported OS"),
                };

                match ffmpeg_install {
                    Ok(_) => println!("ffmpeg is installed on this system"),
                    Err(_) => println!("ffmpeg is not installed on this system"),
                }
            }
        }
    }
}

pub mod cut {
    use std::process::Command;

    pub fn video(start_time: String, end_time: String, input_file: String, output_file: String) {
        let ffmpeg_video = Command::new("ffmpeg")
            .arg("-i")
            .arg(input_file)
            .arg("-ss")
            .arg(start_time)
            .arg("-to")
            .arg(end_time)
            .arg("-c:v")
            .arg("libx264")
            .arg(output_file)
            .output()
            .expect("failed to execute process");
        if !ffmpeg_video.status.success() {
            let error_message = String::from_utf8_lossy(&ffmpeg_video.stderr);
            println!("Error message: {}", error_message);
            return;
        }
    }

    pub fn audio(start_time: String, end_time: String, input_file: String, output_file: String) {
        let ffmpeg_audio = Command::new("ffmpeg")
            .arg("-i")
            .arg(input_file)
            .arg("-ss")
            .arg(start_time)
            .arg("-to")
            .arg(end_time)
            .arg("-acodec")
            .arg("libmp3lame")
            .arg(output_file)
            .output()
            .expect("failed to execute process");

        if !ffmpeg_audio.status.success() {
            let error_message = String::from_utf8_lossy(&ffmpeg_audio.stderr);
            println!("Error message: {}", error_message);
            return;
        }
    }
}

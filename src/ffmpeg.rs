pub mod install {
    use std::process::Command;

    pub fn windows() {
        let ffmpeg_instal = Command::new("powershell")
            .arg("winget")
            .arg("install")
            .arg("ffmpeg")
            .status();
        match ffmpeg_instal {
            Ok(_) => println!("ffmpeg is installed on this system"),
            Err(_) => println!("ffmpeg is not installed on this system"),
        }
    }

    pub fn linux() {
        let ffmpeg_instal = Command::new("apt").arg("install").arg("ffmpeg").status();
        match ffmpeg_instal {
            Ok(_) => println!("ffmpeg is installed on this system"),
            Err(_) => println!("ffmpeg is not installed on this system"),
        }
    }

    pub fn macos() {
        let ffmpeg_instal = Command::new("brew").arg("install").arg("ffmpeg").status();
        match ffmpeg_instal {
            Ok(_) => println!("ffmpeg is installed on this system"),
            Err(_) => println!("ffmpeg is not installed on this system"),
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

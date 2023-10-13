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

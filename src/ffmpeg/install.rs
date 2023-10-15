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
                _ => panic!("Unsupported OS."),
            };

            match ffmpeg_install {
                Ok(_) => println!("ffmpeg is installed on this system."),
                Err(_) => println!("ffmpeg is not installed on this system."),
            }
        }
    }
}

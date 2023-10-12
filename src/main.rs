use std::process::Command;
use which::which;

fn main() {
    if let Ok(_) = which("ffmpeg") {
        // ffmpeg is installed
    } else {
        let system = std::env::consts::OS;
        match system {
            "windows" => install_ffmpeg_windows(),
            "linux" => install_ffmpeg_linux(),
            "macos" => install_ffmpeg_macos(),
            _ => println!("ffmpeg is not installed on this system"),
        }
    }
}

fn install_ffmpeg_windows() {
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

fn install_ffmpeg_linux() {
    let ffmpeg_instal = Command::new("apt").arg("install").arg("ffmpeg").status();
    match ffmpeg_instal {
        Ok(_) => println!("ffmpeg is installed on this system"),
        Err(_) => println!("ffmpeg is not installed on this system"),
    }
}

fn install_ffmpeg_macos() {
    let ffmpeg_instal = Command::new("brew").arg("install").arg("ffmpeg").status();
    match ffmpeg_instal {
        Ok(_) => println!("ffmpeg is installed on this system"),
        Err(_) => println!("ffmpeg is not installed on this system"),
    }
}

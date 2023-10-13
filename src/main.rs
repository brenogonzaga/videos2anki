use which::which;
mod ffmpeg;
mod other;

fn main() {
    match which("ffmpeg") {
        Ok(_) => print!("ffmpeg is already installed"),
        Err(_) => {
            let system = std::env::consts::OS;
            match system {
                "windows" => ffmpeg::install::windows(),
                "linux" => ffmpeg::install::linux(),
                "macos" => ffmpeg::install::macos(),
                _ => println!("ffmpeg is not installed on this system"),
            }
        }
    }
}

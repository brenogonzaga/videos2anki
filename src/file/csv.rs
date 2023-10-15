use std::fs::OpenOptions;
use std::io::Write;

pub fn write(file_name: String, times: Vec<(String, String)>, sentences: Vec<String>) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(format!("./output/{}.csv", file_name))
        .unwrap();

    for (i, (start_time, end_time)) in times.iter().enumerate() {
        let start_time = start_time.replace(":", ".");
        let end_time = end_time.replace(":", ".");
        let video = format!("[sound:{}_{}-{}.mp4]", file_name, start_time, end_time);
        let audio = format!("[sound:{}_{}-{}.mp3]", file_name, start_time, end_time);
        let line = format!("{}\t{}\t{}\t", file_name, video, audio);
        file.write_all(line.as_bytes()).unwrap();

        let line = format!("\t{}\n", sentences[i + 1]);
        file.write_all(line.as_bytes()).unwrap();
    }
}

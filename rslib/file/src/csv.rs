use std::fs::OpenOptions;
use std::io::Write;

pub fn write(
    file_name: &String,
    times: &[(String, String)],
    sentences: &[String],
    output_path: &String,
) {
    let _ = std::fs::create_dir_all(format!("{}/csv", output_path));
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(format!("{}/csv/{}.csv", output_path, file_name))
        .unwrap();

    for (i, (start_time, end_time)) in times.iter().enumerate() {
        let name = (file_name.clone() + start_time + "-" + end_time)
            .replace(' ', "_")
            .replace(':', ".");
        let video = format!("[sound:{}.mp4]", name);
        let audio = format!("[sound:{}.mp3]", name);
        let line = format!("{}\t{}\t{}\t", file_name, video, audio);
        file.write_all(line.as_bytes()).unwrap();

        let line = format!("\t{}\n", sentences[i + 1]);
        file.write_all(line.as_bytes()).unwrap();
    }
}

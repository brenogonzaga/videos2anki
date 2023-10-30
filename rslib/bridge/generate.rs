use file::srt;
use std::{fs::File, io::BufReader};

pub fn times(input_srt: String) -> (Vec<String>, Vec<(String, String)>) {
    let srt_path = File::open(input_srt).unwrap();
    let reader = BufReader::new(srt_path);
    let (sentences, times) = srt::sentences_and_times(reader);
    (sentences, times)
}

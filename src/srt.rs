pub mod extract {
    use regex::Regex;
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    pub fn sentences_and_times(
        reader: BufReader<File>,
    ) -> (Vec<String>, Vec<(String, String)>, usize) {
        let re_sentences = Regex::new(r"\D(.+)").unwrap();
        let re_times =
            Regex::new(r"(\d{2}:\d{2}:\d{2},\d{3}) --> (\d{2}:\d{2}:\d{2},\d{3})").unwrap();
        let mut vec_sentences = Vec::new();
        let mut vec_times = Vec::new();
        let mut sentence = String::new();
        let mut count = 1;

        for line in reader.lines() {
            let line = line.unwrap();

            if line.is_empty() {
                continue;
            }

            if re_times.is_match(&line) {
                let caps = re_times.captures(&line).unwrap();
                let start_time = caps[1].replace(",", ".");
                let end_time = caps[2].replace(",", ".");
                vec_times.push((start_time, end_time));
                vec_sentences.push(sentence.trim().to_string());
                sentence.clear();
                count += 1;
            } else if re_sentences.is_match(&line) {
                sentence.push_str(&line);
            }
        }
        vec_sentences.push(sentence.trim().to_string());
        (vec_sentences, vec_times, count)
    }
}

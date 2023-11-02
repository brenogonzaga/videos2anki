use file::{csv, srt};
use std::{fs::File, io::BufReader};

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) {
    // print the path
    let file = File::open("../test/input.srt");
    let reader = BufReader::new(file.unwrap());
    let (sentences, times) = srt::sentences_and_times(reader);
    let file_name = String::from("output");
    let output_path = String::from("../output");
    csv::WriteCsv::new(file_name, sentences, times, output_path).write();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

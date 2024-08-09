use crate::save_load::general::save_to_filename;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
//Get a string and a point, updates the source of that point
pub fn update_source(project_dir: PathBuf, point_id: String, new_source: String) {
    let mut content: Vec<String> = Vec::new();
    let file_path: PathBuf = [project_dir.clone(), PathBuf::from("Sources.txt")]
        .iter()
        .collect();
    let file = File::open(&file_path).expect("Error while opening file from update_sources");
    for line in BufReader::new(file).lines() {
        if let Ok(l) = line {
            let mut split_line: Vec<String> = l.split("|--|").map(|s| s.to_string()).collect();
            if split_line[0] == point_id.clone() {
                split_line = vec![point_id.clone(), new_source.clone()];
            }
            content.push(split_line.join("|--|"));
        }
    }
    let _ = save_to_filename(
        project_dir.clone(),
        "Sources".to_string(),
        content.join("\n"),
    );
}

//Gets a point, returns its source
pub fn get_point_source(project_dir: PathBuf, point_id: String) -> String {
    let file_path: PathBuf = [project_dir.clone(), PathBuf::from("Sources.txt")]
        .iter()
        .collect();
    let file = File::open(&file_path).expect("Error while opening file from update_sources");
    for line in BufReader::new(file).lines() {
        if let Ok(l) = line {
            let split_line: Vec<String> = l.split("|--|").map(|s| s.to_string()).collect();
            if split_line[0] == point_id.clone() && split_line.len() > 1 {
                return split_line[1].to_string();
            }
        }
    }
    return "No source set yet.".to_string();
}

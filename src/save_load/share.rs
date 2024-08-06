use crate::save_load::general::save_to_filename;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
//Gets a point_id, returns a list with all the bools if it is shared with them or not
pub fn point_is_shared_with(project_dir: PathBuf, point_id: String) -> Vec<bool> {
    let mut result: Vec<bool> = Vec::new();
    let file_path: PathBuf = [project_dir.clone(), PathBuf::from("Library.txt")]
        .iter()
        .collect();
    let file = File::open(&file_path)
        .expect("Error while opening the library file from point_is_shared_with");
    for line in BufReader::new(file).lines() {
        if let Ok(l) = line {
            let split_line: Vec<String> = l.split("@").map(|s| s.to_string()).collect();
            if split_line.len() > 1 {
                result.push(split_line.contains(&point_id));
            }
        }
    }
    return result;
}

//Gets a point_id and a list of titles and bools. If the bool is true it adds the point/confirms it is
//there. If it is false it removes it/confirms the point isn't there.
//Assumes that the length of the checklist and the number of valid lines in library are the same
pub fn share_unshare_point(project_dir: PathBuf, point_id: String, checklist: Vec<bool>) -> () {
    let mut content: Vec<String> = Vec::new();
    let file_path: PathBuf = [project_dir.clone(), PathBuf::from("Library.txt")]
        .iter()
        .collect();
    let file = File::open(&file_path)
        .expect("Error while opening the library file from point_is_shared_with");
    let mut checklist_index: usize = 0;
    for line_read in BufReader::new(file).lines() {
        if let Ok(line) = line_read {
            let mut split_line: Vec<String> = line.split("@").map(|s| s.to_string()).collect();
            if split_line.len() > 1 && split_line[0] != "" {
                if checklist[checklist_index] && !split_line.contains(&point_id) {
                    split_line.push(point_id.clone());
                } else if !checklist[checklist_index] && split_line.contains(&point_id) {
                    split_line.retain(|value| *value != point_id);
                }
                content.push(split_line.join("@"));
                checklist_index += 1;
            }
        }
    }
    let _ = save_to_filename(
        project_dir.clone(),
        "Library".to_string(),
        content.join("\n"),
    );
}

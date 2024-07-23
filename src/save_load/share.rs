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
        let split_line: Vec<String> = line.unwrap().split("@").map(|s| s.to_string()).collect();
        result.push(split_line.contains(&point_id));
    }
    return result;
}

//Gets a point_id and a list of titles and bools. If the bool is true it adds the point/confirms it is
//there. If it is false it removes it/confirms the point isn't there.
pub fn share_unshare_point(
    project_dir: PathBuf,
    point_id: String,
    checklist: Vec<bool>,
    title_list: Vec<String>,
) -> () {
    let mut content: Vec<String> = Vec::new();
    let file_path: PathBuf = [project_dir.clone(), PathBuf::from("Library.txt")]
        .iter()
        .collect();
    let file = File::open(&file_path)
        .expect("Error while opening the library file from point_is_shared_with");
    for (line_read, is_shared, title_id) in BufReader::new(file)
        .lines()
        .into_iter()
        .zip(checklist.into_iter())
        .zip(title_list.into_iter())
        .map(|((x, y), z)| (x, y, z))
    {
        let mut split_line: Vec<String> = line_read
            .unwrap()
            .split("@")
            .map(|s| s.to_string())
            .collect();
        assert_eq!(split_line[0], title_id); //Each line should be referring to a title in the same order
        if is_shared && !split_line.contains(&point_id) {
            split_line.push(point_id.clone());
        } else if !is_shared && split_line.contains(&point_id) {
            split_line.retain(|value| *value != point_id);
        }

        content.push(split_line.join("@"));
    }
    let _ = save_to_filename(
        project_dir.clone(),
        "Library".to_string(),
        content.join("\n"),
    );
}

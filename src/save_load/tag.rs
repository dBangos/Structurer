use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

pub fn get_all_tags(project_dir: PathBuf) -> Vec<String> {
    let mut tag_list: Vec<String> = Vec::new();
    let file_path: PathBuf = [project_dir.clone(), PathBuf::from("Tags.txt")]
        .iter()
        .collect();
    if file_path.exists() {
        let file = File::open(&file_path).expect("Error while opening file from get_all_tags");
        for line in BufReader::new(file).lines() {
            if let Ok(l) = line {
                let split_line: Vec<String> = l.split("@").map(|s| s.to_string()).collect();
                for tag in split_line.into_iter().skip(1) {
                    if tag != "" && !tag_list.contains(&tag) {
                        tag_list.push(tag);
                    }
                }
            }
        }
    }
    return tag_list;
}
pub fn get_title_tags(project_dir: PathBuf, title_id: String) -> Vec<String> {
    let mut tag_list: Vec<String> = Vec::new();
    let file_path: PathBuf = [project_dir.clone(), PathBuf::from("Tags.txt")]
        .iter()
        .collect();
    let file = File::open(&file_path).expect("Error while opening file from get_title_tags");
    for line in BufReader::new(file).lines() {
        if let Ok(l) = line {
            let split_line: Vec<String> = l.split("@").map(|s| s.to_string()).collect();
            if split_line[0] == title_id {
                for tag in split_line.into_iter().skip(1) {
                    if tag != "" && !tag_list.contains(&tag) {
                        tag_list.push(tag);
                    }
                }
            }
        }
    }
    return tag_list;
}

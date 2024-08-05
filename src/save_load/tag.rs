use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
use std::usize;

use crate::Structurer;

//Returns a list of all the tags used in this directory
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

impl Structurer {
    //Adds tags to each title in self.titles
    //This assumes title_ids in the Tags.txt file and in the self struct are in the same order
    pub fn add_tags_to_titles(&mut self) {
        let mut title_index: usize = 0;
        let file_path: PathBuf = [self.project_directory.clone(), PathBuf::from("Tags.txt")]
            .iter()
            .collect();
        if file_path.exists() {
            let file = File::open(&file_path).expect("Error while opening file from get_all_tags");
            for line in BufReader::new(file).lines() {
                if let Ok(l) = line {
                    if title_index < self.titles.len() {
                        let split_line: Vec<String> = l.split("@").map(|s| s.to_string()).collect();
                        if split_line[0] == self.titles[title_index].id && split_line.len() > 1 {
                            for tag in split_line.into_iter().skip(1) {
                                if tag != "" {
                                    self.titles[title_index].tags.push(tag.clone());
                                }
                            }
                            title_index += 1;
                        }
                    } else {
                        //If we've reached the end of the title len, don't waste resources
                        break;
                    }
                }
            }
        }
    }
}

use crate::save_load::general::save_to_filename;
use crate::Title;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
//Returns a vector with all the linked title pairs by index
pub fn get_linked_pairs(project_dir: PathBuf, title_list: Vec<Title>) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    let mut index_id_map: HashMap<String, usize> = HashMap::new();
    for (index, title) in title_list.into_iter().enumerate() {
        index_id_map.insert(title.id, index);
    }
    let all_links = all_titles_links(project_dir.clone());
    for (title, links) in all_links {
        for link in links {
            if !(result.contains(&(index_id_map[&link], index_id_map[&title]))) {
                result.push((index_id_map[&title], index_id_map[&link]));
            }
        }
    }
    return result;
}
//Return a vector with each title_id and all links to it
pub fn all_titles_links(project_dir: PathBuf) -> Vec<(String, Vec<String>)> {
    let mut result: Vec<(String, Vec<String>)> = Vec::new();
    let file_path: PathBuf = [project_dir.clone(), PathBuf::from("Links.txt")]
        .iter()
        .collect();
    if file_path.exists() {
        let file = File::open(&file_path)
            .expect("Error while opening the links file from title_is_linked_with");
        for line in BufReader::new(file).lines() {
            if let Ok(l) = line {
                if l != "" {
                    let split_line: Vec<String> = l.split("|--|").map(|s| s.to_string()).collect();
                    if split_line.len() > 1 && split_line[1] != "" {
                        result.push((split_line[0].to_string(), split_line[1..].to_vec()));
                    } else {
                        result.push((split_line[0].to_string(), vec![]));
                    }
                }
            }
        }
    }
    return result;
}

//Gets a title_id, returns a list with all the bools if it is linked with them or not
pub fn title_is_linked_with(project_dir: PathBuf, title_id: String) -> Vec<bool> {
    let mut result: Vec<bool> = Vec::new();
    let file_path: PathBuf = [project_dir.clone(), PathBuf::from("Links.txt")]
        .iter()
        .collect();
    let file = File::open(&file_path)
        .expect("Error while opening the links file from title_is_linked_with");
    for line in BufReader::new(file).lines() {
        if let Ok(l) = line {
            if l != "" {
                let split_line: Vec<String> = l.split("|--|").map(|s| s.to_string()).collect();
                result.push(split_line.contains(&title_id));
            }
        }
    }
    return result;
}

// Gets a title, a list of titles and bools. Writes or deletes links from Links.txt according to
// bools.
pub fn link_unlink_title(
    project_dir: PathBuf,
    curr_title_index: usize,
    title_list: Vec<Title>,
) -> () {
    let mut content: Vec<String> = Vec::new();
    //Links should have a bool for each title
    assert_eq!(title_list.len(), title_list[curr_title_index].links.len());
    //Creating a list with all the linked title ids
    let mut id_list: Vec<String> = Vec::new();
    for (index, is_linked) in title_list[curr_title_index]
        .links
        .clone()
        .into_iter()
        .enumerate()
    {
        if is_linked && index != curr_title_index {
            id_list.push(title_list[index].id.clone());
        }
    }
    let file_path: PathBuf = [project_dir.clone(), PathBuf::from("Links.txt")]
        .iter()
        .collect();
    let file = File::open(&file_path)
        .expect("Error while opening the links file from point_is_shared_with");
    let mut empty_line_offset: usize = 0;
    for (initial_index, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(l) = line {
            let mut split_line: Vec<String> = l.split("|--|").map(|s| s.to_string()).collect();
            if split_line[0] == "" {
                empty_line_offset += 1;
            } else {
                let index = initial_index - empty_line_offset;
                assert_eq!(split_line[0], title_list[index].id);
                //If this is the line of the title being checked
                if curr_title_index == index {
                    split_line = Vec::new();
                    split_line.push(title_list[curr_title_index].id.clone());
                    for id in id_list.clone() {
                        split_line.push(id);
                    }
                }
                //If this title should be linked
                else if title_list[curr_title_index].links[index] {
                    if !split_line.contains(&title_list[curr_title_index].id.clone()) {
                        split_line.push(title_list[curr_title_index].id.clone());
                    }
                }
                //If this title shouldn't be linked
                else {
                    if split_line.contains(&title_list[curr_title_index].id.clone()) {
                        split_line
                            .retain(|value| *value != title_list[curr_title_index].id.clone());
                    }
                }
                content.push(split_line.join("|--|"));
            }
        }
    }
    let _ = save_to_filename(project_dir.clone(), "Links".to_string(), content.join("\n"));
}

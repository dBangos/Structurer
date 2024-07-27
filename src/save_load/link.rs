use crate::save_load::general::save_to_filename;
use crate::Title;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
//Returns a vector with all the linked title pairs by index
pub fn get_linked_pairs(project_dir: PathBuf) -> Vec<(String, String)> {
    let mut result: Vec<(String, String)> = Vec::new();
    let all_links = all_titles_links(project_dir.clone());
    for (title, links) in all_links {
        for link in links {
            if !(result.contains(&(title.clone(), link.clone()))
                || result.contains(&(link.clone(), title.clone())))
            {
                result.push((title.clone(), link.clone()));
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
            let split_line: Vec<String> = line.unwrap().split("@").map(|s| s.to_string()).collect();
            if split_line.len() > 1 {
                result.push((split_line[0].to_string(), split_line[1..].to_vec()));
            } else {
                result.push((split_line[0].to_string(), vec![]));
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
        let split_line: Vec<String> = line.unwrap().split("@").map(|s| s.to_string()).collect();
        result.push(split_line.contains(&title_id));
    }
    return result;
}

// Gets a title, a list of titles and bools. Writes or deletes links from Links.txt according to
// bools.
pub fn link_unlink_title(
    project_dir: PathBuf,
    curr_title: Title,
    title_id_list: Vec<String>,
) -> () {
    let mut content: Vec<String> = Vec::new();
    let file_path: PathBuf = [project_dir.clone(), PathBuf::from("Links.txt")]
        .iter()
        .collect();
    let file = File::open(&file_path)
        .expect("Error while opening the library file from point_is_shared_with");
    for (line_read, is_shared, title_id) in BufReader::new(file)
        .lines()
        .into_iter()
        .zip(curr_title.links.clone().into_iter())
        .zip(title_id_list.clone().into_iter())
        .map(|((x, y), z)| (x, y, z))
    {
        let mut split_line: Vec<String> = line_read
            .unwrap()
            .split("@")
            .map(|s| s.to_string())
            .collect();
        assert_eq!(split_line[0], title_id); //Each line should be referring to a title in the same order
                                             // On the title line add the ones that should be added, remove the ones that should be
                                             // removed
        if split_line[0] == curr_title.id {
            for (local_is_shared, local_title_id) in curr_title
                .links
                .clone()
                .into_iter()
                .zip(title_id_list.clone().into_iter())
            {
                if local_title_id == curr_title.id {
                    //Ignore the current title so it can't uncheck
                    //itself
                    continue;
                } else if local_is_shared && !split_line.contains(&local_title_id) {
                    split_line.push(local_title_id);
                } else if !local_is_shared && split_line.contains(&local_title_id) {
                    split_line.retain(|value| *value != local_title_id);
                }
            }
        } else if is_shared && !split_line.contains(&curr_title.id) {
            split_line.push(curr_title.id.clone());
        } else if !is_shared && split_line.contains(&curr_title.id) {
            split_line.retain(|value| *value != curr_title.id);
        }

        content.push(split_line.join("@"));
    }
    let _ = save_to_filename(project_dir.clone(), "Links".to_string(), content.join("\n"));
}

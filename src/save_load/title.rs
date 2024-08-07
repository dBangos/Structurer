use crate::save_load::general::{
    delete_all_mentions_from_file, delete_line, delete_line_from_file, insert_line_at_position,
    replace_line, save_to_filename,
};
use crate::save_load::link::{get_linked_pairs, title_is_linked_with};
use crate::save_load::point::delete_point;
use crate::{Structurer, Title};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use uuid::Uuid;

impl Structurer {
    //Gets a title and a position, reorders titles in state and files
    pub fn change_title_position(&mut self, from_position: usize, to_position: usize) {
        self.titles[from_position].links = title_is_linked_with(
            self.project_directory.clone(),
            self.titles[from_position].id.clone(),
        );
        let title = self.titles[from_position].clone();

        //Update the state
        //Wnen dragging below the last element to_position gets len+0 so we have to compensate
        let mut to_position = to_position;
        if to_position >= self.titles.len() {
            to_position = self.titles.len() - 1;
        }
        if from_position < to_position {
            self.titles[from_position..=to_position].rotate_left(1);
        } else {
            self.titles[to_position..=from_position].rotate_right(1);
        }

        //Updating the files
        let lib_file_list = ["Images", "Library", "Tags", "Links"];
        for file_name in lib_file_list {
            let line = delete_line(
                self.project_directory.clone(),
                file_name.to_string(),
                title.id.clone(),
            );
            insert_line_at_position(
                self.project_directory.clone(),
                file_name.to_string(),
                line,
                to_position,
            )
        }
        self.current_title_index = to_position;
        //Updating linked pairs
        self.linked_pairs = get_linked_pairs(self.project_directory.clone(), self.titles.clone());
    }
}

//Adds a title to library and creates the corresponding file
//Returns the new title_id
pub fn add_title(project_dir: PathBuf) -> String {
    let new_id = Uuid::new_v4();
    let mut file_path: PathBuf = [project_dir.clone(), PathBuf::from("Library.txt")]
        .iter()
        .collect();
    let mut file = OpenOptions::new()
        .append(true)
        .open(file_path)
        .expect("Error while opening library file from add_title");
    file.write(("\n".to_string() + &new_id.to_string() + "@New title").as_bytes())
        .expect("Error while writing to library file from add_title");
    file_path = [project_dir.clone(), PathBuf::from("Links.txt")]
        .iter()
        .collect();
    let mut file = OpenOptions::new()
        .append(true)
        .open(file_path)
        .expect("Error while opening links file from add_title");
    file.write(("\n".to_string() + &new_id.to_string()).as_bytes())
        .expect("Error while writing to links file from add_title");
    file_path = [project_dir.clone(), PathBuf::from("Images.txt")]
        .iter()
        .collect();
    let mut file = OpenOptions::new()
        .append(true)
        .open(file_path)
        .expect("Error while opening image file from add_title");
    file.write(("\n".to_string() + &new_id.to_string() + "@").as_bytes())
        .expect("Error while writing to image file from add_title");
    file_path = [project_dir.clone(), PathBuf::from("Tags.txt")]
        .iter()
        .collect();
    let mut file = OpenOptions::new()
        .append(true)
        .open(file_path)
        .expect("Error while opening tags file from add_title");
    file.write(("\n".to_string() + &new_id.to_string() + "@").as_bytes())
        .expect("Error while writing to image file from add_title");
    return new_id.to_string();
}

//Gets a title_id. It deletes the library mention.
//Then it looks if any of the points in that line were only on that line
//if so it deletes them as well and finally it deletes the title file
pub fn delete_title(project_dir: PathBuf, title_id: String) {
    let mut content: Vec<String> = Vec::new();
    let mut deleted_line: Vec<(String, bool)> = Vec::new();
    let file_path: PathBuf = [project_dir.clone(), PathBuf::from("Library.txt")]
        .iter()
        .collect();
    let file = File::open(&file_path).expect("Error while opening file from delete_title");
    for line in BufReader::new(file).lines() {
        if let Ok(l) = line {
            let split_line: Vec<String> = l.split("@").map(|s| s.to_string()).collect();
            if split_line[0] != title_id {
                content.push(split_line.join("@"));
            } else {
                for item in &split_line[2..] {
                    deleted_line.push((item.to_string(), false));
                }
            }
        }
    }
    save_to_filename(
        project_dir.clone(),
        "Library".to_string(),
        content.join("\n"),
    );
    delete_line_from_file(project_dir.clone(), title_id.clone(), "Links".to_string());
    delete_line_from_file(project_dir.clone(), title_id.clone(), "Images".to_string());
    delete_line_from_file(project_dir.clone(), title_id.clone(), "Tags".to_string());
    delete_all_mentions_from_file(project_dir.clone(), title_id.clone(), "Links".to_string());
    let file = File::open(&file_path).expect("Error while opening file from delete_title");
    //Checking for points only on this title
    for line in BufReader::new(file).lines() {
        if let Ok(l) = line {
            let split_line: Vec<String> = l.split("@").map(|s| s.to_string()).collect();
            for (point_id, is_shared) in deleted_line.iter_mut() {
                if *is_shared == false {
                    if split_line.contains(&point_id) {
                        *is_shared = true;
                    }
                }
            }
        }
    }
    //Deleting points only on this title
    for (point_id, is_shared) in deleted_line {
        if !is_shared {
            delete_point(project_dir.clone(), point_id.clone());
        }
    }
}

pub fn save_title(project_dir: PathBuf, title: Title) -> Option<()> {
    if project_dir != PathBuf::new() && title.id != String::new() {
        //Updating the Image file
        let image_string = title.image.path + "@" + &title.image.description;
        replace_line(
            project_dir.clone(),
            title.id.clone(),
            image_string,
            "Images".to_string(),
        );
        //Updating the Tags file
        replace_line(
            project_dir.clone(),
            title.id.clone(),
            title.tags.join("@"),
            "Tags".to_string(),
        );
        //Updating the library file
        let mut content: Vec<String> = Vec::new();
        let file_path: PathBuf = [project_dir.clone(), PathBuf::from("Library.txt")]
            .iter()
            .collect();
        let file =
            File::open(&file_path).expect("Error while opening the library file from save_title");
        for line in BufReader::new(file).lines() {
            if let Ok(l) = line {
                let mut split_line: Vec<String> = l.split("@").map(|s| s.to_string()).collect();
                if split_line[0] == title.id && split_line.len() > 1 {
                    split_line[1] = title.name.clone();
                }
                content.push(split_line.join("@"));
            }
        }
        save_to_filename(
            project_dir.clone(),
            "Library".to_string(),
            content.join("\n"),
        );
        return Some(());
    } else {
        return None;
    }
}

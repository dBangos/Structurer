use crate::save_load::general::{
    delete_all_mentions_from_file, delete_line_from_file, replace_line, save_to_filename,
};
use crate::Title;
use std::fs::OpenOptions;
use std::fs::{remove_file, File};
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use uuid::Uuid;
const VERSION: i32 = 1;
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
    let mut content: Vec<String> = Vec::new();
    content.push("New title".to_string());
    content.push("Version:".to_string() + &VERSION.to_string());
    content.push("Image: ".to_string());
    save_to_filename(project_dir.clone(), new_id.to_string(), content.join("\n"));
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
        .expect("Error while opening links file from add_title");
    file.write(("\n".to_string() + &new_id.to_string()).as_bytes())
        .expect("Error while writing to links file from add_title");
    return new_id.to_string();
}

//Gets a title_id. It deletes the library mention.
//Then it looks if any of the points in that line were only on that line
//if so it deletes them as well and finally it deletes the title file
pub fn delete_title(project_dir: PathBuf, title_id: String) -> () {
    let mut content: Vec<String> = Vec::new();
    let mut deleted_line: Vec<(String, bool)> = Vec::new();
    let mut file_path: PathBuf = [project_dir.clone(), PathBuf::from("Library.txt")]
        .iter()
        .collect();
    let file = File::open(&file_path).expect("Error while opening file from delete_title");
    for line in BufReader::new(file).lines() {
        let split_line: Vec<String> = line.unwrap().split("@").map(|s| s.to_string()).collect();
        if split_line[0] != title_id {
            content.push(split_line.join("@"));
        } else {
            for item in &split_line[2..] {
                deleted_line.push((item.to_string(), false));
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
    delete_all_mentions_from_file(project_dir.clone(), title_id.clone(), "Links".to_string());
    let file = File::open(&file_path).expect("Error while opening file from delete_title");
    //Checking for points only on this title
    for line in BufReader::new(file).lines() {
        let split_line: Vec<String> = line.unwrap().split("@").map(|s| s.to_string()).collect();
        for (point_id, is_shared) in deleted_line.iter_mut() {
            if *is_shared == false {
                if split_line.contains(&point_id) {
                    *is_shared = true;
                }
            }
        }
    }
    //Deleting points only on this title
    for (point_id, is_shared) in deleted_line {
        if !is_shared {
            file_path = [
                project_dir.clone(),
                PathBuf::from(point_id.clone() + ".txt"),
            ]
            .iter()
            .collect();
            let _ = remove_file(file_path);
        }
    }
    file_path = [
        project_dir.clone(),
        PathBuf::from(title_id.clone() + ".txt"),
    ]
    .iter()
    .collect();
    let _ = remove_file(file_path);
}
//Changes the title in a title_id file. The title is always in the first line, so the first line
//just gets overwritten
pub fn save_title(project_dir: PathBuf, title: Title) -> () {
    //Open the file-> Read its content->Modify the proper title->Save contents in old files' place
    let mut content: Vec<String> = Vec::new();
    content.push(title.name.clone());
    content.push("Version:".to_string() + &VERSION.to_string());
    content.push("Image@".to_string() + &title.image.path + "@" + &title.image.description);
    save_to_filename(project_dir.clone(), title.id.clone(), content.join("\n"));
    //Updating the Image file
    replace_line(
        project_dir.clone(),
        title.id.clone(),
        title.image.path,
        "Images".to_string(),
    );
    //Updating the library file
    let mut content: Vec<String> = Vec::new();
    let file_path: PathBuf = [project_dir.clone(), PathBuf::from("Library.txt")]
        .iter()
        .collect();
    let file = File::open(&file_path)
        .expect("Error while opening the library file from change_title_name");
    for line in BufReader::new(file).lines() {
        let mut split_line: Vec<String> = line.unwrap().split("@").map(|s| s.to_string()).collect();
        if split_line[0] == title.id {
            split_line[1] = title.name.clone();
        }
        content.push(split_line.join("@"));
    }
    let _ = save_to_filename(
        project_dir.clone(),
        "Library".to_string(),
        content.join("\n"),
    );
}

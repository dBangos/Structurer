use crate::save_load::image::{get_point_images, get_title_image};
use crate::save_load::link::title_is_linked_with;
use crate::save_load::point::{get_point_content_from_file, save_point};
use crate::save_load::title::save_title;
use crate::{Point, Structurer, Title};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
//Gets file, line and element. Appends element to the line
pub fn add_element_to_line(
    project_dir: PathBuf,
    line_identifier: String,
    element: String,
    file_name: String,
) -> () {
    let mut content: Vec<String> = Vec::new();
    let file_path: PathBuf = [
        project_dir.clone(),
        PathBuf::from(file_name.clone() + ".txt"),
    ]
    .iter()
    .collect();
    //Open the file-> Read its content->Modify the proper title->Save contents in old files' place
    let file = File::open(&file_path).expect("Error while opening file from add_element_to_line");
    for line in BufReader::new(file).lines() {
        let mut split_line: Vec<String> = line.unwrap().split("@").map(|s| s.to_string()).collect();
        if split_line[0] == line_identifier {
            split_line.push(element.to_string());
        }
        content.push(split_line.join("@"));
    }
    let _ = save_to_filename(
        project_dir.clone(),
        file_name.to_string(),
        content.join("\n"),
    );
}
//Gets a line and a file, deletes line starting with identifier from file
pub fn delete_line_from_file(project_dir: PathBuf, identifier: String, file_name: String) {
    let mut content: Vec<String> = Vec::new();
    let file_path: PathBuf = [
        project_dir.clone(),
        PathBuf::from(file_name.clone() + ".txt"),
    ]
    .iter()
    .collect();
    let file = File::open(&file_path).expect("Error while opening file from delete_line_from_file");
    for line in BufReader::new(file).lines() {
        let split_line: Vec<String> = line.unwrap().split("@").map(|s| s.to_string()).collect();
        if split_line[0].to_string() != identifier {
            content.push(split_line.join("@"));
        }
    }
    save_to_filename(
        project_dir.clone(),
        file_name.to_string(),
        content.join("\n"),
    );
}
//Deletes all mentions of string from the file
pub fn delete_all_mentions_from_file(
    project_dir: PathBuf,
    identifier: String,
    file_name: String,
) -> () {
    let mut content: Vec<String> = Vec::new();
    let file_path: PathBuf = [
        project_dir.clone(),
        PathBuf::from(file_name.clone() + ".txt"),
    ]
    .iter()
    .collect();
    //Open the file-> Read its content->Modify the proper title->Save contents in old files' place
    let file = File::open(&file_path)
        .expect("Error while opening file from delete_all_mentions_from_file");
    for line in BufReader::new(file).lines() {
        let split_line: Vec<String> = line
            .unwrap()
            .split("@")
            .map(|s| s.to_string())
            .filter(|s| *s != identifier)
            .collect();
        content.push(split_line.join("@"));
    }
    let _ = save_to_filename(project_dir.clone(), file_name, content.join("\n"));
}
//Gets file, line and element. Deletes line and replace is with new line
pub fn replace_line(
    project_dir: PathBuf,
    line_identifier: String,
    element: String,
    file_name: String,
) -> () {
    let mut content: Vec<String> = Vec::new();
    let file_path: PathBuf = [
        project_dir.clone(),
        PathBuf::from(file_name.clone() + ".txt"),
    ]
    .iter()
    .collect();
    //Open the file-> Read its content->Modify the proper title->Save contents in old files' place
    let file = File::open(&file_path).expect("Error while opening file from add_element_to_line");
    for line in BufReader::new(file).lines() {
        let mut split_line: Vec<String> = line.unwrap().split("@").map(|s| s.to_string()).collect();
        if split_line[0] == line_identifier {
            split_line = Vec::new();
            split_line.push(line_identifier.clone());
            split_line.push(element.to_string());
        }
        content.push(split_line.join("@"));
    }
    let _ = save_to_filename(
        project_dir.clone(),
        file_name.to_string(),
        content.join("\n"),
    );
}
//Gets a file name and path, saves content to it.
pub fn save_to_filename(project_dir: PathBuf, id: String, content: String) -> () {
    let file_path: PathBuf = [project_dir, PathBuf::from(id + ".txt")].iter().collect();
    let mut file =
        File::create(&file_path).expect("Error while creating file from save_to_filename");
    let _ = file.write_all(content.as_bytes());
}
//Gets the filename of a txt file, returns its content.
pub fn load_from_filename(title: String, project_dir: PathBuf) -> String {
    let file_path: PathBuf = [project_dir, PathBuf::from(title + ".txt")]
        .iter()
        .collect();
    let mut file =
        File::open(&file_path).expect("Error while opening file from load_from_filename");
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", file_path.display(), why),
        Ok(_) => return s,
    }
}

impl Structurer {
    //Loading the titles and corresponding points from the Libary.txt file.
    //This file has a title_id being the first word of each line
    //the title being the second word,
    //followed by the "@" symbol befgre each point.
    pub fn load_from_library(&mut self) -> () {
        let file_path: PathBuf = [self.project_directory.clone(), PathBuf::from("Library.txt")]
            .iter()
            .collect();
        if file_path.exists() {
            let file =
                File::open(&file_path).expect("Error while opening file from load_from_library");
            self.title_order = Vec::new();
            self.titles = HashMap::new();
            for line in BufReader::new(file).lines() {
                let split_line: Vec<String> =
                    line.unwrap().split("@").map(|s| s.to_string()).collect();
                if split_line.len() > 2 {
                    let mut temp_title: Title = Title::default();
                    temp_title.id = split_line[0].clone();
                    temp_title.name = split_line[1].clone();
                    temp_title.point_ids = split_line[2..].to_vec();
                    self.title_order.push(temp_title.id.clone());
                    self.titles
                        .insert(temp_title.id.clone(), temp_title.clone());
                }
            }
        }
        //Loading the image data so it can be shown in the node view
        let file_path: PathBuf = [self.project_directory.clone(), PathBuf::from("Images.txt")]
            .iter()
            .collect();
        if file_path.exists() {
            let file = File::open(&file_path)
                .expect("Error while opening Images file from load_from_library");
            for line in BufReader::new(file).lines() {
                let split_line: Vec<String> =
                    line.unwrap().split("@").map(|s| s.to_string()).collect();
                if split_line.len() == 2 && split_line[0] != "" {
                    self.titles.get_mut(&split_line[0]).unwrap().image.path = split_line[1].clone();
                }
            }
        }
    }
}
//Helper function that saves and updates state
//Turned this into a function instead of a method on Structurerto avoid borrow conflicts
pub fn save_old_add_new_points(
    project_directory: PathBuf,
    current_title: Title,
    current_points: Vec<Point>,
    new_title: Title,
) -> (Title, Vec<Point>) {
    //Saving the title of the curent page before switching
    //First checking if the file exists
    let temp_file_path_for_check: PathBuf = [
        project_directory.clone(),
        PathBuf::from(current_title.id.clone() + ".txt"),
    ]
    .iter()
    .collect();
    if temp_file_path_for_check.exists() {
        save_title(project_directory.clone(), current_title.clone());
    }
    let mut return_current_points: Vec<Point> = Vec::new();
    let mut return_title = new_title.clone();
    //Updating the links for the new title_id
    return_title.links = title_is_linked_with(project_directory.clone(), new_title.id.clone());
    return_title.image = get_title_image(project_directory.clone(), new_title.id);
    //Save old points => Remove old points => Add new points
    for point in current_points.clone() {
        save_point(project_directory.clone(), point);
    }
    for new_point_id in new_title.point_ids.into_iter() {
        let mut new_point: Point = Point::default();
        new_point.id = new_point_id.to_string();
        new_point.content =
            get_point_content_from_file(project_directory.clone(), new_point.clone());
        new_point.images = get_point_images(project_directory.clone(), new_point_id.clone());
        return_current_points.push(new_point);
    }
    return (return_title, return_current_points);
}

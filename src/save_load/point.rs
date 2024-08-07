use crate::save_load::general::{
    add_element_to_line, delete_all_mentions_from_file, delete_line_from_file, load_from_filename,
    save_to_filename,
};
use crate::{Point, Structurer};
use std::fs::OpenOptions;
use std::fs::{remove_file, File};
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use std::usize;
use uuid::Uuid;

use super::general::replace_line;

impl Structurer {
    pub fn change_point_position(&mut self, from_position: usize, to_position: usize) {
        //Update the state
        //Wnen dragging below the last element to_position gets len+0 so we have to compensate
        let mut to_position = to_position;
        if to_position >= self.titles[self.current_title_index].point_ids.len() {
            to_position = self.titles[self.current_title_index].point_ids.len() - 1;
        }
        if from_position < to_position {
            self.titles[self.current_title_index].point_ids[from_position..=to_position]
                .rotate_left(1);
        } else {
            self.titles[self.current_title_index].point_ids[to_position..=from_position]
                .rotate_right(1);
        }
        //Update the library file
        let new_line = self.titles[self.current_title_index].name.clone()
            + "@"
            + &self.titles[self.current_title_index].point_ids.join("@");
        replace_line(
            self.project_directory.clone(),
            self.titles[self.current_title_index].id.clone(),
            new_line,
            "Library".to_string(),
        );
        //Reloading the points
        self.current_points = load_points_from_title_id(
            self.project_directory.clone(),
            self.titles[self.current_title_index].id.clone(),
        );
    }
}
//Adds a point to the current page/title, create the corresponding file and adds it to the library.
//Returns a tuple(id,content)
pub fn add_point(project_dir: PathBuf, title_id: String) -> Option<Point> {
    if title_id != String::new() && project_dir != PathBuf::new() {
        let id = Uuid::new_v4();
        save_to_filename(project_dir.clone(), id.to_string(), "New point".to_string());
        add_element_to_line(
            project_dir.clone(),
            title_id,
            id.to_string(),
            "Library".to_string(),
        );

        let file_path: PathBuf = [project_dir.clone(), PathBuf::from("Sources.txt")]
            .iter()
            .collect();
        let mut file = OpenOptions::new()
            .append(true)
            .open(file_path)
            .expect("Error while opening sources file from add_point");
        file.write(("\n".to_string() + &id.to_string() + "@").as_bytes())
            .expect("Error while writing to sourcse file from add_point");
        let mut new_point: Point = Point::default();
        new_point.id = id.to_string();
        new_point.content = "New point".to_string();
        return Some(new_point);
    } else {
        return None;
    }
}
//Gets a point id, deletes the corresponding file and all library mentions
pub fn delete_point(project_dir: PathBuf, point_id: String) -> () {
    let file_path: PathBuf = [
        project_dir.clone(),
        PathBuf::from(point_id.clone() + ".txt"),
    ]
    .iter()
    .collect();
    let _ = remove_file(file_path);
    delete_all_mentions_from_file(project_dir.clone(), point_id.clone(), "Library".to_string());
    delete_line_from_file(project_dir.clone(), point_id.clone(), "Sources".to_string());
}
pub fn get_point_content_from_file(project_dir: PathBuf, point_id: String) -> String {
    let mut result_string: String = String::new();
    let file_path: PathBuf = [
        project_dir.clone(),
        PathBuf::from(point_id.clone() + ".txt"),
    ]
    .iter()
    .collect();
    let file =
        File::open(&file_path).expect("Error while opening file from get_point_content_from_file");
    for line in BufReader::new(file).lines() {
        if let Ok(l) = line {
            let split_line: Vec<String> = l.split("@").map(|s| s.to_string()).collect();
            if split_line[0] != "Image" {
                result_string = result_string + &split_line.join("@") + "\n";
            }
        }
    }
    return result_string;
}

pub fn save_point(project_dir: PathBuf, point: Point) {
    let mut content: Vec<String> = Vec::new();
    for image in point.images {
        let new_string: String = "Image@".to_string() + &image.path + "@" + &image.description;
        content.push(new_string);
    }
    content.push(point.content);
    let _ = save_to_filename(
        project_dir.clone(),
        point.id.to_string(),
        content.join("\n"),
    );
}

//Gets a title_id, loads the corresponding point_ids and point_content
pub fn load_points_from_title_id(project_dir: PathBuf, title_id: String) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::new();
    let mut library_line: Vec<String> = Vec::new();
    let file_path: PathBuf = [project_dir.clone(), PathBuf::from("Library.txt")]
        .iter()
        .collect();
    let file =
        File::open(&file_path).expect("Error while opening file from load_points_from_title_id");
    for line in BufReader::new(file).lines() {
        if let Ok(l) = line {
            let split_line: Vec<String> = l.split("@").map(|s| s.to_string()).collect();
            if split_line[0] == title_id {
                library_line = split_line[2..].to_vec();
                break;
            }
        }
    }
    for point in library_line.into_iter() {
        let mut new_point: Point = Point::default();
        new_point.id = point.clone();
        new_point.content = get_point_content_from_file(project_dir.clone(), point.clone());
        new_point.content = load_from_filename(point, project_dir.clone());
        result.push(new_point);
    }
    return result;
}

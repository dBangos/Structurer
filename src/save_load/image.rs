use crate::save_load::general::save_to_filename;
use crate::ImageStruct;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
pub fn add_image_to_point(project_dir: PathBuf, point_id: String, image: ImageStruct) {
    let mut image_added: bool = false;
    let mut content: Vec<String> = Vec::new();
    let file_path: PathBuf = [
        project_dir.clone(),
        PathBuf::from(point_id.clone() + ".txt"),
    ]
    .iter()
    .collect();
    let file = File::open(&file_path).expect("Error while opening file from add_image_to_point");
    for line in BufReader::new(file).lines() {
        let split_line: Vec<String> = line.unwrap().split("@").map(|s| s.to_string()).collect();
        if split_line[0] == "Image" {
            continue;
        } else if image_added == false {
            let new_image_line = "Image@".to_string() + &image.path + "@" + &image.description;
            content.push(new_image_line);
            image_added = true;
        }
        content.push(split_line.join("@"));
    }
    save_to_filename(
        project_dir.clone(),
        point_id.to_string(),
        content.join("\n"),
    );
}
pub fn delete_image_from_point(project_dir: PathBuf, point_id: String, image: ImageStruct) {
    let mut image_removed: bool = false;
    let mut content: Vec<String> = Vec::new();
    let file_path: PathBuf = [
        project_dir.clone(),
        PathBuf::from(point_id.clone() + ".txt"),
    ]
    .iter()
    .collect();
    let file =
        File::open(&file_path).expect("Error while opening file from delete_image_from_point");
    for line in BufReader::new(file).lines() {
        let split_line: Vec<String> = line.unwrap().split("@").map(|s| s.to_string()).collect();
        //If the line contains the requested image, don't push it to content
        if split_line.len() == 3 {
            if image_removed == false && split_line[0] == "Image" && split_line[1] == image.path {
                image_removed = true;
            }
        } else {
            content.push(split_line.join("@"));
        }
    }
    let _ = save_to_filename(
        project_dir.clone(),
        point_id.to_string(),
        content.join("\n"),
    );
}

pub fn get_point_images(project_dir: PathBuf, point_id: String) -> Vec<ImageStruct> {
    let mut return_vec: Vec<ImageStruct> = Vec::new();
    let file_path: PathBuf = [
        project_dir.clone(),
        PathBuf::from(point_id.clone() + ".txt"),
    ]
    .iter()
    .collect();
    let file =
        File::open(&file_path).expect("Error while opening file from delete_image_from_point");
    for line in BufReader::new(file).lines() {
        let split_line: Vec<String> = line.unwrap().split("@").map(|s| s.to_string()).collect();
        if split_line.len() == 3 {
            if split_line[0] == "Image" {
                let mut new_image: ImageStruct = ImageStruct::default();
                new_image.path = split_line[1].clone();
                new_image.description = split_line[2].clone();
                return_vec.push(new_image);
            }
        }
    }
    return return_vec;
}
pub fn get_title_image(project_dir: PathBuf, title_id: String) -> ImageStruct {
    let mut return_image = ImageStruct::default();
    let file_path: PathBuf = [
        project_dir.clone(),
        PathBuf::from(title_id.clone() + ".txt"),
    ]
    .iter()
    .collect();
    let file = File::open(&file_path).expect("Error while opening file from get_title_image");
    for line in BufReader::new(file).lines() {
        let split_line: Vec<String> = line.unwrap().split("@").map(|s| s.to_string()).collect();
        if split_line[0] == "Image" && split_line.len() == 3 {
            return_image.path = split_line[1].clone();
            return_image.description = split_line[2].clone();
            break;
        }
    }
    return return_image;
}

use crate::save_load::general::save_to_filename;
use crate::ImageStruct;
use crate::Structurer;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use std::thread;

impl Structurer<'_> {
    pub fn load_title_images(&mut self) {
        self.title_images_map = HashMap::new();
        let mut handles = Vec::new();
        for title in self.titles.clone() {
            let handle = thread::spawn(move || {
                let file_path = title.image.path.clone();
                let image = egui::Image::new(format!("file://{file_path}"));
                (title.id, image)
            });
            handles.push(handle);
        }
        for handle in handles {
            let (key, val) = handle.join().unwrap();
            self.title_images_map.insert(key, val);
        }
    }
}

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
        if let Ok(l) = line {
            let split_line: Vec<String> = l.split("|--|").map(|s| s.to_string()).collect();
            if split_line[0] == "Image" {
                continue;
            } else if image_added == false {
                let new_image_line =
                    "Image|--|".to_string() + &image.path + "|--|" + &image.description;
                content.push(new_image_line);
                image_added = true;
            }
            content.push(split_line.join("|--|"));
        }
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
        if let Ok(l) = line {
            let split_line: Vec<String> = l.split("|--|").map(|s| s.to_string()).collect();
            //If the line contains the requested image, don't push it to content
            if split_line.len() == 3 {
                if image_removed == false && split_line[0] == "Image" && split_line[1] == image.path
                {
                    image_removed = true;
                }
            } else {
                content.push(split_line.join("|--|"));
            }
        }
    }
    let _ = save_to_filename(
        project_dir.clone(),
        point_id.to_string(),
        content.join("\n"),
    );
}

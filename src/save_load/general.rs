use crate::save_load::link::get_linked_pairs;
use crate::save_load::link::title_is_linked_with;
use crate::save_load::point::save_point;
use crate::save_load::tag::get_all_tags;
use crate::save_load::title::save_title;
use crate::{Structurer, Title};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use std::usize;

//Gets file, line and element. Appends element to the line
pub fn add_element_to_line(
    project_dir: PathBuf,
    line_identifier: &str,
    element: &str,
    file_name: &str,
) -> () {
    let mut content: Vec<String> = Vec::new();
    let file_path: PathBuf = [
        project_dir.clone(),
        PathBuf::from(file_name.to_owned() + ".txt"),
    ]
    .iter()
    .collect();
    //Open the file-> Read its content->Modify the proper title->Save contents in old files' place
    let file = File::open(&file_path).expect("Error while opening file from add_element_to_line");
    for line in BufReader::new(file).lines() {
        if let Ok(l) = line {
            let mut split_line: Vec<&str> = l.split("|--|").collect();
            if split_line[0] == line_identifier {
                split_line.push(element);
            }
            content.push(split_line.join("|--|"));
        }
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
        if let Ok(l) = line {
            let split_line: Vec<String> = l.split("|--|").map(|s| s.to_string()).collect();
            if split_line[0].to_string() != identifier {
                content.push(split_line.join("|--|"));
            }
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
        if let Ok(l) = line {
            let split_line: Vec<String> = l
                .split("|--|")
                .map(|s| s.to_string())
                .filter(|s| *s != identifier)
                .collect();
            content.push(split_line.join("|--|"));
        }
    }
    let _ = save_to_filename(project_dir.clone(), file_name, content.join("\n"));
}

//Gets already created string line a file and a N number. Adds the line after N lines
pub fn insert_line_at_position(
    project_dir: PathBuf,
    file_name: String,
    new_line: String,
    position: usize,
) {
    let mut content: Vec<String> = Vec::new();
    let file_path: PathBuf = [
        project_dir.clone(),
        PathBuf::from(file_name.clone() + ".txt"),
    ]
    .iter()
    .collect();
    let mut current_line: usize = 0;
    let mut line_inserted: bool = false;
    //Open the file-> Read its content->Don't read line starting with id->Save content
    let file = File::open(&file_path).expect("Error while opening file from delete_line");
    for line in BufReader::new(file).lines() {
        if let Ok(l) = line {
            let split_line: Vec<String> = l.split("|--|").map(|s| s.to_string()).collect();
            if split_line[0] != "" {
                if current_line == position && line_inserted == false {
                    content.push(new_line.clone());
                    line_inserted = true;
                }
                content.push(split_line.join("|--|"));
                current_line += 1;
            }
        }
    }
    if line_inserted == false {
        content.push(new_line);
    }
    let _ = save_to_filename(
        project_dir.clone(),
        file_name.to_string(),
        content.join("\n"),
    );
}

//Gets file and identifier. Removes line starting with identifier
pub fn delete_line(project_dir: PathBuf, file_name: String, line_identifier: String) -> String {
    let mut content: Vec<String> = Vec::new();
    let file_path: PathBuf = [
        project_dir.clone(),
        PathBuf::from(file_name.clone() + ".txt"),
    ]
    .iter()
    .collect();
    let mut deleted_line: String = String::new();
    //Open the file-> Read its content->Don't read line starting with id->Save content
    let file = File::open(&file_path).expect("Error while opening file from delete_line");
    for line in BufReader::new(file).lines() {
        if let Ok(l) = line {
            let split_line: Vec<String> = l.split("|--|").map(|s| s.to_string()).collect();
            if split_line[0] != line_identifier {
                content.push(split_line.join("|--|"));
            } else {
                deleted_line = split_line.join("|--|");
            }
        }
    }
    let _ = save_to_filename(
        project_dir.clone(),
        file_name.to_string(),
        content.join("\n"),
    );
    return deleted_line;
}

//Gets file, line and element. Deletes line and replace is with new line
pub fn replace_line(
    project_dir: PathBuf,
    line_identifier: &str,
    element: &str,
    file_name: &str,
) -> () {
    let mut content: Vec<String> = Vec::new();
    let file_path: PathBuf = [
        project_dir.clone(),
        PathBuf::from(file_name.to_owned() + ".txt"),
    ]
    .iter()
    .collect();
    //Open the file-> Read its content->Modify the proper title->Save contents in old files' place
    let file = File::open(&file_path).expect("Error while opening file from replace_line");
    for line in BufReader::new(file).lines() {
        if let Ok(l) = line {
            let mut split_line: Vec<&str> = l.split("|--|").collect();
            if split_line[0] == line_identifier {
                split_line = Vec::new();
                split_line.push(line_identifier);
                split_line.push(element);
            }
            content.push(split_line.join("|--|"));
        }
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

impl Structurer {
    pub fn change_title(&mut self, index: usize) {
        if self.center_current_node {
            self.drag_distance = -1.0 * self.titles[index].node_physics_position * self.view_scale;
        }
        save_title(
            self.project_directory.clone(),
            self.titles[self.current_title_index].clone(),
        );
        self.next_page_point_ids = self.titles[index].point_ids.clone();
        self.save_old_add_new_points();
        self.current_title_index = index;
        self.titles[index].links = title_is_linked_with(
            self.project_directory.clone(),
            self.titles[index].id.clone(),
        );
    }

    //Loading the titles and corresponding points from the Libary.txt file.
    //This file has a title_id being the first word of each line
    //the title being the second word,
    //followed by the "|--|" symbol befgre each point.
    pub fn load_from_library(&mut self) -> () {
        let file_path: PathBuf = [self.project_directory.clone(), PathBuf::from("Library.txt")]
            .iter()
            .collect();
        if file_path.exists() {
            let file =
                File::open(&file_path).expect("Error while opening file from load_from_library");
            self.titles = Vec::new();
            for line in BufReader::new(file).lines() {
                if let Ok(l) = line {
                    let split_line: Vec<String> = l.split("|--|").map(|s| s.to_string()).collect();
                    if split_line.len() > 1 {
                        let mut temp_title: Title = Title::default();
                        temp_title.id = split_line[0].clone();
                        temp_title.name = split_line[1].clone();
                        if split_line.len() > 2 {
                            temp_title.point_ids = split_line[2..].to_vec();
                        }
                        self.titles.push(temp_title.clone());
                    }
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
                if let Ok(l) = line {
                    let split_line: Vec<String> = l.split("|--|").map(|s| s.to_string()).collect();
                    if split_line.len() == 3 && split_line[0] != "" {
                        //If this is too slow replace it with a hashmap
                        for title in self.titles.iter_mut() {
                            if title.id == split_line[0] {
                                title.image.path = split_line[1].clone();
                                title.image.description = split_line[2].clone();
                                break;
                            }
                        }
                    }
                }
            }
        }
        self.linked_pairs = get_linked_pairs(self.project_directory.clone(), self.titles.clone());
        self.all_tags = get_all_tags(self.project_directory.clone());
        self.tags_actively_filtering = vec![false; self.all_tags.len()];
        self.add_tags_to_titles();
    }

    //Helper function that saves and updates state
    //Turned this into a function instead of a method on Structurerto avoid borrow conflicts
    pub fn save_old_add_new_points(&mut self) {
        //Saving the title of the curent page before switching
        for id in self.current_point_ids.clone() {
            save_point(self.project_directory.clone(), self.points[&id].clone());
        }
        self.current_point_ids = self.next_page_point_ids.clone();
        self.next_page_point_ids = Vec::new();
    }
}

use crate::{Point, Structurer, Title};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::fs::{remove_file, File};
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use uuid::Uuid;
const VERSION: i32 = 1;

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
        let split_line: Vec<String> = line.unwrap().split("@").map(|s| s.to_string()).collect();
        if split_line[0] == title_id {
            library_line = split_line[2..].to_vec();
            break;
        }
    }
    for point in library_line.into_iter() {
        let mut new_point: Point = Point::default();
        new_point.id = point.clone();
        new_point.content = load_from_filename(point, project_dir.clone());
        result.push(new_point);
    }
    return result;
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
}
//Gets a file name and path, saves content to it.
pub fn save_to_filename(project_dir: PathBuf, id: String, content: String) -> () {
    let file_path: PathBuf = [project_dir, PathBuf::from(id + ".txt")].iter().collect();
    let mut file =
        File::create(&file_path).expect("Error while creating file from save_to_filename");
    let _ = file.write_all(content.as_bytes());
}

//Adds a point to the current page/title, creates the corresponding file and adds it to the library.
//Returns a tuple(id,content)
pub fn add_point(project_dir: PathBuf, title_id: String) -> Point {
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
    file.write(("\n".to_string() + &id.to_string()).as_bytes())
        .expect("Error while writing to sourcse file from add_point");
    let mut new_point: Point = Point::default();
    new_point.id = id.to_string();
    new_point.content = "New point".to_string();
    return new_point;
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

//Changes the title in a title_id file. The title is always in the first line, so the first line
//just gets overwritten
pub fn change_title_name(project_dir: PathBuf, title_id: String, new_title: String) -> () {
    let file_path: PathBuf = [
        project_dir.clone(),
        PathBuf::from(title_id.clone() + ".txt"),
    ]
    .iter()
    .collect();

    //Open the file-> Read its content->Modify the proper title->Save contents in old files' place
    let file = File::open(&file_path).expect("Error while opening file from change_title_name");
    let mut first_line: bool = true;
    let mut content: Vec<String> = Vec::new();
    for line in BufReader::new(file).lines() {
        if first_line == true {
            content.push(new_title.to_string());
            first_line = false;
        } else {
            content.push(line.expect("Error while reading a title file."));
        }
    }
    save_to_filename(project_dir.clone(), title_id.clone(), content.join("\n"));
    //Updating the library file
    let mut content: Vec<String> = Vec::new();
    let file_path: PathBuf = [project_dir.clone(), PathBuf::from("Library.txt")]
        .iter()
        .collect();
    let file = File::open(&file_path)
        .expect("Error while opening the library file from change_title_name");
    for line in BufReader::new(file).lines() {
        let mut split_line: Vec<String> = line.unwrap().split("@").map(|s| s.to_string()).collect();
        if split_line[0] == title_id {
            split_line[1] = new_title.clone();
        }
        content.push(split_line.join("@"));
    }
    let _ = save_to_filename(
        project_dir.clone(),
        "Library".to_string(),
        content.join("\n"),
    );
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
    let mut content: Vec<String> = Vec::new();
    content.push("New title".to_string());
    content.push("Version:".to_string() + &VERSION.to_string());
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

//Gets a point_id and a list of titles and bools. If the bool is true it adds the point/confirms it is
//there. If it is false it removes it/confirms the point isn't there.
pub fn share_unshare_point(
    project_dir: PathBuf,
    point_id: String,
    checklist: Vec<bool>,
    title_list: Vec<String>,
) -> () {
    let mut content: Vec<String> = Vec::new();
    let file_path: PathBuf = [project_dir.clone(), PathBuf::from("Library.txt")]
        .iter()
        .collect();
    let file = File::open(&file_path)
        .expect("Error while opening the library file from point_is_shared_with");
    for (line_read, is_shared, title_id) in BufReader::new(file)
        .lines()
        .into_iter()
        .zip(checklist.into_iter())
        .zip(title_list.into_iter())
        .map(|((x, y), z)| (x, y, z))
    {
        let mut split_line: Vec<String> = line_read
            .unwrap()
            .split("@")
            .map(|s| s.to_string())
            .collect();
        assert_eq!(split_line[0], title_id); //Each line should be referring to a title in the same order
        if is_shared && !split_line.contains(&point_id) {
            split_line.push(point_id.clone());
        } else if !is_shared && split_line.contains(&point_id) {
            split_line.retain(|value| *value != point_id);
        }

        content.push(split_line.join("@"));
    }
    let _ = save_to_filename(
        project_dir.clone(),
        "Library".to_string(),
        content.join("\n"),
    );
}

//Gets a point_id, returns a list with all the bools if it is shared with them or not
pub fn point_is_shared_with(project_dir: PathBuf, point_id: String) -> Vec<bool> {
    let mut result: Vec<bool> = Vec::new();
    let file_path: PathBuf = [project_dir.clone(), PathBuf::from("Library.txt")]
        .iter()
        .collect();
    let file = File::open(&file_path)
        .expect("Error while opening the library file from point_is_shared_with");
    for line in BufReader::new(file).lines() {
        let split_line: Vec<String> = line.unwrap().split("@").map(|s| s.to_string()).collect();
        result.push(split_line.contains(&point_id));
    }
    return result;
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

//Get a string and a point, updates the source of that point
pub fn update_source(project_dir: PathBuf, point_id: String, new_source: String) {
    let mut content: Vec<String> = Vec::new();
    let file_path: PathBuf = [project_dir.clone(), PathBuf::from("Sources.txt")]
        .iter()
        .collect();
    let file = File::open(&file_path).expect("Error while opening file from update_sources");
    for line in BufReader::new(file).lines() {
        let mut split_line: Vec<String> = line.unwrap().split("@").map(|s| s.to_string()).collect();
        if split_line[0] == point_id.clone() {
            split_line = vec![point_id.clone(), new_source.clone()];
        }
        content.push(split_line.join("@"));
    }
    let _ = save_to_filename(
        project_dir.clone(),
        "Sources".to_string(),
        content.join("\n"),
    );
}

//Gets a point, returns its source
pub fn get_point_source(project_dir: PathBuf, point_id: String) -> String {
    let file_path: PathBuf = [project_dir.clone(), PathBuf::from("Sources.txt")]
        .iter()
        .collect();
    let file = File::open(&file_path).expect("Error while opening file from update_sources");
    for line in BufReader::new(file).lines() {
        let split_line: Vec<String> = line.unwrap().split("@").map(|s| s.to_string()).collect();
        if split_line[0] == point_id.clone() && split_line.len() > 1 {
            return split_line[1].to_string();
        }
    }
    return "No source set yet.".to_string();
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

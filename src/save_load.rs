//Gets a title_id, loads the corresponding point_ids and point_content
use std::fs::{remove_file, File};
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use uuid::Uuid;

pub fn load_points_from_title_id(project_dir: PathBuf, title_id: String) -> Vec<(String, String)> {
    let mut result: Vec<(String, String)> = Vec::new();
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
        result.push((
            point.clone(),
            load_from_filename(point.clone(), project_dir.clone()),
        ));
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

//Loading the titles and corresponding points from the Libary.txt file.
//This file has a title_id being the first word of each line
//the title being the second word,
//followed by the "@" symbol befgre each point.
pub fn load_from_library(project_dir: PathBuf) -> Vec<(String, String, Vec<String>)> {
    let file_path: PathBuf = [project_dir, PathBuf::from("Library.txt")].iter().collect();
    let file = File::open(&file_path).expect("Error while opening file from load_from_library");
    let mut result: Vec<(String, String, Vec<String>)> = Vec::new();
    for line in BufReader::new(file).lines() {
        let split_line: Vec<String> = line.unwrap().split("@").map(|s| s.to_string()).collect();
        result.push((
            split_line[0].clone(),
            split_line[1].clone(),
            split_line[2..].to_vec(),
        ));
    }
    return result;
}

//Gets a title_id, returns tags and field information
//fn load_from_title_id(project_dir: PathBuf, title:String)->Vec<String>{
//
//}

//Gets a file name and path, saves content to it.
pub fn save_to_filename(project_dir: PathBuf, id: String, content: String) -> () {
    let file_path: PathBuf = [project_dir, PathBuf::from(id + ".txt")].iter().collect();
    let mut file =
        File::create(&file_path).expect("Error while creating file from save_to_filename");
    let _ = file.write_all(content.as_bytes());
}

//Adds a point to the passed title in the Library.txt file
pub fn add_point_to_library(project_dir: PathBuf, title_id: String, point_id: String) -> () {
    let mut content: Vec<String> = Vec::new();
    let file_path: PathBuf = [project_dir.clone(), PathBuf::from("Library.txt")]
        .iter()
        .collect();
    //Open the file-> Read its content->Modify the proper title->Save contents in old files' place
    let file = File::open(&file_path).expect("Error while opening file from add_point_to_library");
    for line in BufReader::new(file).lines() {
        let mut split_line: Vec<String> = line.unwrap().split("@").map(|s| s.to_string()).collect();
        if split_line[0] == title_id {
            split_line.push(point_id.to_string());
        }
        content.push(split_line.join("@"));
    }
    let _ = save_to_filename(
        project_dir.clone(),
        "Library".to_string(),
        content.join("\n"),
    );
}

//Adds a point to the current page/title, creates the corresponding file and adds it to the library.
//Returns a tuple(id,content)
pub fn add_point(project_dir: PathBuf, title_id: String) -> (String, String) {
    let id = Uuid::new_v4();
    save_to_filename(project_dir.clone(), id.to_string(), "New point".to_string());
    add_point_to_library(project_dir.clone(), title_id, id.to_string());
    return (id.to_string(), "New point".to_string());
}

//Deletes all mentions of point_id from the library file
pub fn delete_point_from_library(project_dir: PathBuf, point_id: String) -> () {
    let mut content: Vec<String> = Vec::new();
    let file_path: PathBuf = [project_dir.clone(), PathBuf::from("Library.txt")]
        .iter()
        .collect();
    //Open the file-> Read its content->Modify the proper title->Save contents in old files' place
    let file =
        File::open(&file_path).expect("Error while opening file from delete_point_from_library");
    for line in BufReader::new(file).lines() {
        let split_line: Vec<String> = line
            .unwrap()
            .split("@")
            .map(|s| s.to_string())
            .filter(|s| *s != point_id)
            .collect();
        content.push(split_line.join("@"));
    }
    let _ = save_to_filename(
        project_dir.clone(),
        "Library".to_string(),
        content.join("\n"),
    );
}

//Gets a point id, deletes the corresponding file and all library mentions
pub fn delete_point(project_dir: PathBuf, point_id: String) -> () {
    println!("Delete with pointid{}", point_id.clone());
    let file_path: PathBuf = [
        project_dir.clone(),
        PathBuf::from(point_id.clone() + ".txt"),
    ]
    .iter()
    .collect();
    let _ = remove_file(file_path);
    delete_point_from_library(project_dir.clone(), point_id.clone());
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
pub fn add_title(project_dir: PathBuf) -> () {
    let mut content: Vec<String> = Vec::new();
    let new_id = Uuid::new_v4();
    let file_path: PathBuf = [project_dir.clone(), PathBuf::from("Library.txt")]
        .iter()
        .collect();
    //Open the file-> Read its content->Modify the proper title->Save contents in old files' place
    let file = File::open(&file_path).expect("Error while opening file from add_title");
    for line in BufReader::new(file).lines() {
        content.push(line.expect("Error while reading lines in add_title"));
    }
    content.push(new_id.to_string() + "@New title");
    save_to_filename(
        project_dir.clone(),
        "Library".to_string(),
        content.join("\n"),
    );
    let content = "New title".to_string();
    save_to_filename(project_dir.clone(), new_id.to_string(), content);
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
    let file = File::open(&file_path).expect("Error while opening file from delete_title");
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
    checklist: Vec<(String, String, bool)>,
) -> () {
    let mut content: Vec<String> = Vec::new();
    let file_path: PathBuf = [project_dir.clone(), PathBuf::from("Library.txt")]
        .iter()
        .collect();
    let file = File::open(&file_path)
        .expect("Error while opening the library file from point_is_shared_with");
    for (line_read, (title_id, _title, is_shared)) in BufReader::new(file)
        .lines()
        .into_iter()
        .zip(checklist.into_iter())
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

//Gets a point_id, returns a list with all the titles and if it is shared with them or not
pub fn point_is_shared_with(project_dir: PathBuf, point_id: String) -> Vec<(String, String, bool)> {
    let mut result: Vec<(String, String, bool)> = Vec::new();
    let file_path: PathBuf = [project_dir.clone(), PathBuf::from("Library.txt")]
        .iter()
        .collect();

    let file = File::open(&file_path)
        .expect("Error while opening the library file from point_is_shared_with");
    for line in BufReader::new(file).lines() {
        let split_line: Vec<String> = line.unwrap().split("@").map(|s| s.to_string()).collect();
        result.push((
            split_line[0].clone(),
            split_line[1].clone(),
            split_line.contains(&point_id),
        ));
    }
    return result;
}

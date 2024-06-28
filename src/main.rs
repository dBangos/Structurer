use eframe::egui;
use rfd::MessageDialogResult;
use std::fs::{remove_file, File};
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use uuid::Uuid;

struct Structurer {
    project_directory: PathBuf,
    titles_points: Vec<(String, String, Vec<String>)>, //Titles_points(title, title_id ,corresponding_points)
    current_points: Vec<(String, String)>,             //Current_point(point_id,point_content)
    current_title: String,
    current_title_id: String,
    age: i32,
}

impl Default for Structurer {
    fn default() -> Self {
        Self {
            project_directory: Default::default(),
            titles_points: Vec::new(), //Titles_points(title, title_id ,corresponding_points)
            current_points: Vec::new(), //Current_point(point_id,point_content)
            current_title: String::new(),
            current_title_id: String::new(),
            age: 40,
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 1000.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<Structurer>::default()
        }),
    )
}

impl eframe::App for Structurer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");

            //Button Line
            ui.horizontal(|ui| {
                if ui.button("Set Project Directory").clicked() {
                    if let Some(dir_path) = rfd::FileDialog::new().pick_folder() {
                        self.project_directory = dir_path;
                    }
                    self.titles_points = load_from_library(self.project_directory.clone());
                }
                if ui.button("Save").clicked() {
                    change_title_name(
                        self.project_directory.clone(),
                        self.current_title_id.clone(),
                        self.current_title.clone(),
                    );
                    for (id, content) in self.current_points.clone().into_iter() {
                        save_to_filename(self.project_directory.clone(), id, content);
                    }
                }
                if ui.button("New").clicked() {
                    self.age += 1;
                }
                if ui.button("Add Point").clicked() {
                    self.current_points.push(add_point(
                        self.project_directory.clone(),
                        self.current_title_id.clone(),
                    ));
                    self.titles_points = load_from_library(self.project_directory.clone());
                }
                if ui.button("Add Title").clicked() {
                    add_title(self.project_directory.clone());
                }
                if ui.button("Delete Title").clicked() {
                    delete_title();
                }
                if ui.button("Save Page As:").clicked() {
                    self.age += 1;
                }
            });

            //Main layout, contains titles layout and points layout
            ui.horizontal(|ui| {
                //Titles layout ==========================================================
                ui.vertical(|ui| {
                    if ui.button("Filler").clicked() {
                        self.age += 1;
                    }
                    if ui.button("Filler").clicked() {
                        self.age += 1;
                    }
                    //Making sure tha data is clean
                    let temp_file_path_for_check: PathBuf =
                        [self.project_directory.clone(), PathBuf::from("Library.txt")]
                            .iter()
                            .collect();
                    if temp_file_path_for_check.exists() {
                        self.titles_points = load_from_library(self.project_directory.clone());
                    }
                    //Binding each title button to loading the corresponding points
                    for (title_id, title, t_points) in self.titles_points.iter_mut() {
                        if ui.button(title.clone()).clicked() {
                            //Saving the title of the curent page before switching
                            //First checking if the file exists
                            let temp_file_path_for_check: PathBuf = [
                                self.project_directory.clone(),
                                PathBuf::from(self.current_title_id.clone() + ".txt"),
                            ]
                            .iter()
                            .collect();
                            if temp_file_path_for_check.exists() {
                                change_title_name(
                                    self.project_directory.clone(),
                                    self.current_title_id.clone(),
                                    self.current_title.clone(),
                                );
                            }
                            //Setting the title field
                            self.current_title = title.clone();
                            self.current_title_id = title_id.clone();
                            //Save old points => Remove old points => Add new points
                            for (id, content) in self.current_points.clone().into_iter() {
                                save_to_filename(self.project_directory.clone(), id, content);
                            }
                            self.current_points = Vec::new();
                            for new_point in t_points.into_iter() {
                                self.current_points.push((
                                    new_point.to_string(),
                                    load_from_filename(
                                        new_point.to_string(),
                                        self.project_directory.clone(),
                                    ),
                                ));
                            }
                        }
                    }
                });

                //All points layout==========================================
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        let name_label = ui.label("Your name: ");
                        ui.text_edit_singleline(&mut self.current_title)
                            .labelled_by(name_label.id);
                    });
                    //Updates the current_points variable, this is how point deletions get updated
                    //and shown on the ui
                    let temp_file_path_for_check: PathBuf =
                        [self.project_directory.clone(), PathBuf::from("Library.txt")]
                            .iter()
                            .collect();
                    if temp_file_path_for_check.exists() {
                        self.current_points = load_points_from_title_id(
                            self.project_directory.clone(),
                            self.current_title_id.clone(),
                        );
                    }
                    for point in self.current_points.iter_mut() {
                        // Container for elements of each point
                        ui.horizontal(|ui| {
                            if ui.button("Delete").clicked() {
                                let message_dialog_result = rfd::MessageDialog::new()
                                    .set_title(
                                        "Are you sure you want to permanently delete this point?",
                                    )
                                    .set_buttons(rfd::MessageButtons::YesNo)
                                    .show();
                                if message_dialog_result == MessageDialogResult::Yes {
                                    delete_point(self.project_directory.clone(), point.0.clone());
                                    self.titles_points =
                                        load_from_library(self.project_directory.clone());
                                    ctx.request_repaint();
                                }
                            }
                            ui.text_edit_multiline(&mut point.1);
                        });
                    }
                });
            });

            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                self.age += 1;
            }
        });
    }
}

//Gets a title_id, loads the corresponding point_ids and point_content
fn load_points_from_title_id(project_dir: PathBuf, title_id: String) -> Vec<(String, String)> {
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
fn load_from_filename(title: String, project_dir: PathBuf) -> String {
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
fn load_from_library(project_dir: PathBuf) -> Vec<(String, String, Vec<String>)> {
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
fn save_to_filename(project_dir: PathBuf, id: String, content: String) -> () {
    let file_path: PathBuf = [project_dir, PathBuf::from(id + ".txt")].iter().collect();
    let mut file =
        File::create(&file_path).expect("Error while creating file from save_to_filename");
    let _ = file.write_all(content.as_bytes());
}

//Adds a point to the proper place in the Library.txt file
fn add_point_to_library(project_dir: PathBuf, title_id: String, point_id: String) -> () {
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
fn add_point(project_dir: PathBuf, title_id: String) -> (String, String) {
    let id = Uuid::new_v4();
    save_to_filename(project_dir.clone(), id.to_string(), "New point".to_string());
    add_point_to_library(project_dir.clone(), title_id, id.to_string());
    return (id.to_string(), "New point".to_string());
}

//Deletes all mentions of point_id from the library file
fn delete_point_from_library(project_dir: PathBuf, point_id: String) -> () {
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
fn delete_point(project_dir: PathBuf, point_id: String) -> () {
    println!("Delete with pointid{}", point_id.clone());
    let file_path: PathBuf = [
        project_dir.clone(),
        PathBuf::from(point_id.clone() + ".txt"),
    ]
    .iter()
    .collect();
    remove_file(file_path);
    delete_point_from_library(project_dir.clone(), point_id.clone());
}

//Changes the title in a title_id file. The title is always in the first line, so the first line
//just gets overwritten
fn change_title_name(project_dir: PathBuf, title_id: String, new_title: String) -> () {
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
        .expect("Error while opening the library file form change_title_name");
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
fn add_title(project_dir: PathBuf) -> () {
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
fn delete_title() -> () {}

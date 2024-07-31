use crate::save_load::general::save_to_filename;
use crate::save_load::link::get_linked_pairs;
use crate::Structurer;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::io::Read;
use std::{fs, path::PathBuf};
#[derive(Serialize, Deserialize)]
struct Config {
    project_directory: PathBuf,
}

impl Structurer {
    //Everything that needs to get the program ready at startup
    pub fn start_routine(&mut self) -> () {
        //Check if there is a Structurer directory and if so read the config
        //If not, create the directory and file
        let mut dir_path: PathBuf = [dirs::config_dir().unwrap(), PathBuf::from("Structurer")]
            .iter()
            .collect();
        if !dir_path.exists() {
            let _ = fs::create_dir(dir_path.clone());
        }
        dir_path.push("Structurer_state.json");
        if dir_path.exists() {
            let mut file = fs::File::open(&dir_path).expect("Error while opening config file");
            let mut buff = String::new();
            file.read_to_string(&mut buff).unwrap();
            let new_config: Config = serde_json::from_str(&buff).unwrap();
            self.project_directory = new_config.project_directory;
            self.load_from_library();
        }
        self.linked_pairs = get_linked_pairs(self.project_directory.clone(), self.titles.clone());
    }

    //Saving stuff to the config file in the default OS location
    pub fn save_to_config(&mut self) -> Result<()> {
        let current_config = Config {
            project_directory: self.project_directory.clone(),
        };
        let dir_path: PathBuf = [
            dirs::config_dir().unwrap(),
            PathBuf::from("Structurer/Structurer_state.json"),
        ]
        .iter()
        .collect();
        let file_content = serde_json::to_string_pretty(&current_config)?;
        let _ = fs::write(dir_path, file_content);

        Ok(())
    }

    pub fn create_library_links(&mut self) {
        //Creating a library file if there isn't one
        let file_path: PathBuf = [self.project_directory.clone(), PathBuf::from("Library.txt")]
            .iter()
            .collect();
        if !file_path.exists() {
            save_to_filename(
                self.project_directory.clone(),
                "Library".to_string(),
                "".to_string(),
            )
        }
        //Creating a links file if there isn't one
        let file_path: PathBuf = [self.project_directory.clone(), PathBuf::from("Links.txt")]
            .iter()
            .collect();
        if !file_path.exists() {
            save_to_filename(
                self.project_directory.clone(),
                "Links".to_string(),
                "".to_string(),
            )
        }
    }
}

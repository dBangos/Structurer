use crate::save_load::general::save_to_filename;
use crate::Structurer;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::io::Read;
use std::{fs, path::PathBuf};
#[derive(Serialize, Deserialize)]
struct Config {
    project_directory: PathBuf,
    //title_loaded: bool,
    //current_title_index: usize,
    center_current_node: bool,
    node_view_start_stop_physics: bool,
    stop_clicked_nodes: bool,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            project_directory: PathBuf::default(),
            //title_loaded: false,
            //current_title_index: 0,
            center_current_node: true,
            node_view_start_stop_physics: true,
            stop_clicked_nodes: false,
        }
    }
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
        let mut file = fs::File::open(&dir_path).expect("Error while opening config file");
        let mut buff = String::new();
        file.read_to_string(&mut buff).unwrap();
        let new_config: Config = serde_json::from_str(&buff).unwrap_or_default();
        self.project_directory = new_config.project_directory;
        //self.title_loaded = new_config.title_loaded;
        //self.current_title_index = new_config.current_title_index;
        self.center_current_node = new_config.center_current_node;
        self.node_view_start_stop_physics = new_config.node_view_start_stop_physics;
        self.stop_clicked_nodes = new_config.stop_clicked_nodes;
        self.load_from_library();
        self.get_all_points();
    }

    //Saving stuff to the config file in the default OS location
    pub fn save_to_config(&mut self) -> Result<()> {
        let current_config = Config {
            project_directory: self.project_directory.clone(),
            //title_loaded: self.title_loaded,
            //current_title_index: self.current_title_index,
            center_current_node: self.center_current_node,
            node_view_start_stop_physics: self.node_view_start_stop_physics,
            stop_clicked_nodes: self.stop_clicked_nodes,
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

    //If a library file doesn't exist, create it
    pub fn create_library_files(&mut self) {
        let file_vec: Vec<&str> = vec!["Library", "Sources", "Images", "Links", "Tags"];
        for file_name in file_vec {
            let file_path: PathBuf = [
                self.project_directory.clone(),
                PathBuf::from(file_name.to_string() + ".txt"),
            ]
            .iter()
            .collect();
            if !file_path.exists() {
                save_to_filename(
                    self.project_directory.clone(),
                    file_name.to_string(),
                    "".to_string(),
                )
            }
        }
    }
}

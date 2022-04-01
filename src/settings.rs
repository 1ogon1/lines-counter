use serde::{Deserialize, Serialize};
use std::{env, fs, path::Path};

use crate::utils::StringExtensions;

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub(crate) output: String,
    pub(crate) directory: String,

    pub(crate) include_commets: bool,

    //if array is empty, includes all files
    pub(crate) extensions: Vec<String>,
}

impl Settings {
    pub fn new() -> Self {
        let settings_json =
            fs::read_to_string(Settings::get_path()).expect("Error: can't find settings.json");

        serde_json::from_str::<Settings>(&settings_json).expect("Error: can't parse settings")
    }

    pub(crate) fn is_allowed_file(&self, file_name: &str) -> bool {
        if self.extensions.len() == 0 {
            true
        } else {
            self.extensions.iter().any(|ex| file_name.ends_with(ex))
        }
    }

    pub(crate) fn validate(&self) {
        assert!(self.directory.len() > 0, "Error: project_dir is empty");
        assert!(
            Path::new(&self.directory.get_root_path()).exists(),
            "Error: project_dir doesn't exist"
        );
    }

    fn get_path() -> String {
        format!(
            "{}/{}",
            env::current_dir()
                .expect("Can't get project dir")
                .to_str()
                .unwrap(),
            "settings.json"
        )
    }
}

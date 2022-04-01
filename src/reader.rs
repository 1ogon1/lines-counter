use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::path::Path;
use std::{collections::HashMap, vec};

use crate::{settings::Settings, utils::StringExtensions};

#[derive(Serialize, Deserialize)]
struct File {
    name: String,
    lines: u64,
}
#[derive(Serialize, Deserialize)]
pub struct Reader {
    settings: Settings,
    folders: HashMap<String, Vec<File>>,
}

impl Reader {
    pub fn new() -> Self {
        Reader {
            folders: HashMap::new(),
            settings: Settings::new(),
        }
    }

    pub fn read(&mut self) {
        self.settings.validate();

        let root = self.settings.directory.get_root_path();

        self.read_dir(&root);
        self.save_to_file();
    }

    fn read_dir(&mut self, directory: &str) {
        let content = std::fs::read_dir(directory).unwrap();

        for item in content {
            match item.as_ref() {
                Ok(data) => {
                    if let Ok(metadata) = data.metadata() {
                        let path = data.path().display().to_string();

                        if metadata.is_dir() {
                            self.read_dir(&path);
                        } else if metadata.is_file() && self.settings.is_allowed_file(&path) {
                            let files = self
                                .folders
                                .entry(path.get_pure_directory(&self.settings.directory))
                                .or_insert(vec![]);
                            let file = File {
                                name: path.clone().get_file_name(),
                                lines: path.get_file_lines(&self.settings),
                            };

                            files.push(file);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn save_to_file(&self) {
        if Path::new(&self.settings.output).exists() {
            std::fs::remove_file(&self.settings.output).unwrap();
        }

        let mut file = std::fs::File::create(&self.settings.output).unwrap();

        file.write_all(serde_json::to_string(&self.folders).unwrap().as_ref())
            .unwrap();
    }
}

use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::{prelude::*, BufReader},
    path::Path,
    vec,
};

use crate::settings::ExcludeCommentsType;
use crate::{settings::Settings, utils::StringExtensions};

#[derive(Serialize, Deserialize)]
struct File {
    name: String,
    lines: u64,
}

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
        let root = self.settings.directory.get_root_path();

        self.read_dir(&root);
        self.save_to_file();
    }

    fn read_dir(&mut self, directory: &str) {
        let content = std::fs::read_dir(directory).unwrap();

        for item in content {
            if let Ok(data) = item {
                if let Ok(metadata) = data.metadata() {
                    let path = data.path().display().to_string();

                    if metadata.is_dir() {
                        self.read_dir(&path);
                    } else if metadata.is_file() && self.settings.is_allowed_file(&path) {
                        let file_lines = self.get_file_lines(&path);
                        let files = self
                            .folders
                            .entry(path.get_pure_directory(&self.settings.directory))
                            .or_insert(vec![]);
                        let file = File {
                            name: path.clone().get_file_name(),
                            lines: file_lines,
                        };

                        files.push(file);
                    }
                }
            }
        }
    }

    fn get_file_lines(&self, file_path: &str) -> u64 {
        let mut count = 0u64;
        let mut multiline_comment = false;
        let file = std::fs::File::open(file_path).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                let line = line.trim();
                if line.len() > 0 {
                    if let Some(comment_type) = self.settings.get_comment_type(&line) {
                        if comment_type == ExcludeCommentsType::Multiline {
                            multiline_comment = !multiline_comment;
                            if self.settings.can_count(line) {
                                count += 1;
                            }
                        }
                    } else {
                        if !multiline_comment {
                            count += 1;
                        }
                    }
                } else if !self.settings.exclude_empty_line && !multiline_comment {
                    count += 1;
                }
            }
        }

        count
    }

    fn save_to_file(&self) {
        let path = "output.json";
        if Path::new(path).exists() {
            std::fs::remove_file(path).unwrap();
        }

        let mut file = std::fs::File::create(path).unwrap();

        file.write_all(serde_json::to_string(&self.folders).unwrap().as_ref())
            .unwrap();
    }
}

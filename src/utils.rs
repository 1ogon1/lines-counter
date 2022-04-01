use crate::settings::Settings;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub trait StringExtensions {
    fn get_root_path(&self) -> String;
    fn get_file_name(&self) -> String;
    fn get_pure_directory(&self, pattern: &str) -> String;

    fn get_file_lines(&self, settings: &Settings) -> u64;
}

impl StringExtensions for String {
    fn get_root_path(&self) -> String {
        let home = dirs::home_dir().expect("Can't get home directory");

        format!("{}{}", home.to_str().unwrap(), &self)
    }

    fn get_file_name(&self) -> String {
        self[self.rfind('/').unwrap_or(0) + 1..self.len()].to_string()
    }

    fn get_pure_directory(&self, pattern: &str) -> String {
        self[self.find(pattern).unwrap_or(0)..self.rfind('/').unwrap_or(self.len())].to_string()
    }

    fn get_file_lines(&self, settings: &Settings) -> u64 {
        let mut count = 0u64;
        let file = File::open(self).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                let line = line.trim();
                if line.len() > 0 {
                    if line.starts_with("//") {
                        count = if settings.include_commets {
                            count + 1
                        } else {
                            count
                        };
                    } else {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

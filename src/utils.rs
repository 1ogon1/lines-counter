pub trait StringExtensions {
    fn get_root_path(&self) -> String;
    fn get_file_name(&self) -> String;
    fn get_pure_directory(&self, pattern: &str) -> String;
}

impl StringExtensions for String {
    fn get_root_path(&self) -> String {
        let home = dirs::home_dir().expect("Error: get home directory");

        if self.contains(home.to_str().unwrap()) {
            self.to_string()
        } else {
            format!("{}{}", home.to_str().unwrap(), self)
        }
    }

    fn get_file_name(&self) -> String {
        self[self.rfind('/').unwrap_or(0) + 1..self.len()].to_string()
    }

    fn get_pure_directory(&self, pattern: &str) -> String {
        self[self.find(pattern).unwrap_or(0)..self.rfind('/').unwrap_or(self.len())].to_string()
    }
}

use serde::{Deserialize, Serialize};
use std::{env, fs, path::Path};

use crate::utils::StringExtensions;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ExcludeCommentsType {
    All,
    Inline,
    Multiline,
    NotExclude,
}

#[derive(Serialize, Deserialize)]
pub struct MultilineCommentFormat {
    pub(crate) begin: String,
    pub(crate) end: String,
}

#[derive(Serialize, Deserialize)]
pub struct CommentFormat {
    pub(crate) intile: String,
    pub(crate) multiline: MultilineCommentFormat,
}

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub(crate) directory: String,

    pub(crate) exclude_empty_line: bool,

    pub(crate) comment_format: CommentFormat,
    pub(crate) exclude_comments: ExcludeCommentsType,
    // If array is empty, includes all files
    pub(crate) extensions: Vec<String>,
}

impl Settings {
    pub fn new() -> Self {
        let settings_json =
            fs::read_to_string(Settings::get_path()).expect("Error: can't find settings.json");

        let mut this =
            serde_json::from_str::<Settings>(&settings_json).expect("Error: can't parse settings");

        if this.directory.len() == 0 {
            this.directory = get_current_project_directory();
        }

        assert!(
            Path::new(&this.directory.get_root_path()).exists(),
            "Error: project_dir doesn't exist"
        );

        this
    }

    pub(crate) fn is_allowed_file(&self, file_name: &str) -> bool {
        if self.extensions.len() == 0 {
            true
        } else {
            self.extensions.iter().any(|ex| file_name.ends_with(ex))
        }
    }

    pub(crate) fn get_comment_type(&self, line: &str) -> Option<ExcludeCommentsType> {
        match self.exclude_comments {
            ExcludeCommentsType::All => {
                if self.has_inline_comment(line) {
                    Some(ExcludeCommentsType::Inline)
                } else {
                    self.has_multiline_comment(line)
                }
            }
            ExcludeCommentsType::Inline => {
                if self.has_inline_comment(line) {
                    Some(ExcludeCommentsType::Inline)
                } else {
                    None
                }
            }
            ExcludeCommentsType::Multiline => self.has_multiline_comment(line),
            ExcludeCommentsType::NotExclude => None,
        }
    }

    pub(crate) fn can_count(&self, line: &str) -> bool {
        if let Some(index) = line.find(&self.comment_format.multiline.begin) {
            index != 0
        } else if let Some(index) = line.find(&self.comment_format.multiline.end) {
            index != line.len() - self.comment_format.multiline.end.len()
        } else {
            false
        }
    }

    fn get_path() -> String {
        format!(
            "{}/{}",
            env::current_dir()
                .expect("Error: get project dir")
                .to_str()
                .unwrap(),
            "settings.json"
        )
    }

    fn has_inline_comment(&self, line: &str) -> bool {
        line.starts_with(&self.comment_format.intile)
    }

    fn has_multiline_comment(&self, line: &str) -> Option<ExcludeCommentsType> {
        let begin = line.contains(&self.comment_format.multiline.begin);
        let end = line.contains(&self.comment_format.multiline.end);

        if begin && end {
            let len = line.len() - self.comment_format.multiline.end.len();
            let end_at = line.find(&self.comment_format.multiline.end).unwrap();
            let start_at = line.find(&self.comment_format.multiline.begin).unwrap();

            if start_at == 0 && end_at == len {
                Some(ExcludeCommentsType::Inline)
            } else {
                None
            }
        } else {
            if begin || end {
                Some(ExcludeCommentsType::Multiline)
            } else {
                None
            }
        }
    }
}

fn get_current_project_directory() -> String {
    if let Ok(path) = env::current_dir() {
        path.to_str()
            .expect("Error: get current directory")
            .to_string()
    } else {
        panic!("Error: get directory");
    }
}

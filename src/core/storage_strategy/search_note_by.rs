use std::fmt::Display;

use colored::Colorize;
use itertools::Itertools;

#[derive(Debug, Clone)]
pub enum SearchNoteBy {
    Title(String),
    Filename(String),
    Content(Vec<String>),
    Tags(Vec<String>),
}

impl Display for SearchNoteBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchNoteBy::Title(title) => {
                f.write_fmt(format_args!("{} '{}'", "title".blue(), title.purple()))
            }
            SearchNoteBy::Filename(filename) => f.write_fmt(format_args!(
                "{} '{}'",
                "filename".blue(),
                filename.purple()
            )),
            SearchNoteBy::Content(content) => f.write_fmt(format_args!(
                "{} {}",
                "content".blue(),
                content
                    .iter()
                    .map(|item| format!("{}", item.purple()))
                    .join(", ")
            )),
            SearchNoteBy::Tags(tags) => f.write_fmt(format_args!(
                "{} {}",
                "tags".blue(),
                tags.iter()
                    .map(|item| format!("{}", item.purple()))
                    .join(", ")
            )),
        }
    }
}

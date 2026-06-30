use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum SearchNoteBy {
    Title(String),
    Filename(String),
    All,
}

impl Display for SearchNoteBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchNoteBy::Title(title) => f.write_fmt(format_args!("Title {title}")),
            SearchNoteBy::Filename(filename) => f.write_fmt(format_args!("Filename {filename}")),
            SearchNoteBy::All => f.write_fmt(format_args!("No criteria")),
        }
    }
}

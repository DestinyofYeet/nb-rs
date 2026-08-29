use crate::core::models::note::Note;
use colored::Colorize;
use itertools::Itertools;

impl std::fmt::Display for Note<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tags = self.get_metadata().get_tags();
        f.write_fmt(format_args!(
            "{}{} {}",
            self.get_title().blue(),
            if !tags.is_empty() {
                format!(" [{}]", tags.iter().map(|e| e.purple()).join(", "))
            } else {
                "".to_string()
            },
            format!("({})", self.get_file_name()).white(),
        ))
    }
}

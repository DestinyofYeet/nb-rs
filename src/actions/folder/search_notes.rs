use colored::Colorize;
use thiserror::Error;

use crate::actions::{
    folder::{list::ListFolderError, model::Folder},
    note::model::{Note, NoteError},
};

#[derive(Error, Debug)]
pub enum SearchNotesError {
    #[error(transparent)]
    ListFolder(#[from] ListFolderError),

    #[error(transparent)]
    ReadError(#[from] NoteError),
}

pub struct SearchNoteResult {
    pub note: Note,
    pub snippets: Vec<String>,
}

impl Folder {
    pub fn search_notes_content(
        &self,
        term: &str,
    ) -> Result<Vec<SearchNoteResult>, SearchNotesError> {
        let entries = self.list()?;
        let mut matching_notes: Vec<SearchNoteResult> = Vec::new();
        for note in entries.notes.iter() {
            let lines = note.get_content_by_lines()?;
            let mut search_result = SearchNoteResult {
                note: note.clone(),
                snippets: Vec::new(),
            };

            let mut line_before: Option<String> = None;
            let mut line_number = 0;

            for line in lines.map_while(Result::ok) {
                line_number += 1;

                if line.to_lowercase().contains(term) {
                    let index = line.to_lowercase().find(term).unwrap();
                    let (_, correct_word) = line.split_at(index);
                    let (correct_word, _) = correct_word.split_at(term.len());
                    let correct_word_color = format!("{}", correct_word.red());
                    let line = line.replace(correct_word, &correct_word_color);

                    let snippet = {
                        let snippet = format!(
                            "  {}{} {}",
                            line_number.to_string().green(),
                            ":".blue(),
                            line
                        );

                        match line_before.take() {
                            None => snippet,
                            Some(line_before) => {
                                format!(
                                    "  {}{} {}\n{}",
                                    (line_number - 1).to_string().green(),
                                    ":".blue(),
                                    line_before,
                                    snippet
                                )
                            }
                        }
                    };
                    search_result.snippets.push(snippet);
                }

                if line_number != 1 {
                    line_before = Some(line);
                }
            }

            if !search_result.snippets.is_empty() {
                matching_notes.push(search_result);
            }
        }

        for folder in entries.folders.iter() {
            let mut sub_notes = folder.search_notes_content(term)?;
            matching_notes.append(&mut sub_notes);
        }
        Ok(matching_notes)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        actions::{folder::model::Folder, note::model::Note},
        tests::test::Test,
    };

    #[test]
    fn get_notes_by_name() {
        let test = Test::setup("get_notes_by_name");

        let root_folder = Folder::from_pathbuf(&test.dir, ".").unwrap();

        Note::new_create(test.dir.to_str().unwrap(), "test.md").unwrap();

        let folder = Folder::from_pathbuf(&test.dir, "a_folder").unwrap();
        folder.create().unwrap();

        Note::new_create(folder.get_path().to_str().unwrap(), "nested_test.md").unwrap();

        let results = root_folder.get_notes_by_name("test").unwrap();

        assert_eq!(
            format!("{results:?}"),
            r#"[Note { path: "/tmp/nb-rs_test_dir/get_notes_by_name", name: "test.md" }, Note { path: "/tmp/nb-rs_test_dir/get_notes_by_name/a_folder", name: "nested_test.md" }]"#
        )
    }
}

use std::path::Path;

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

    #[error("failed to convert '{}' to a number: {}", .input, .err)]
    Convert { input: String, err: String },

    #[error("{} is an invalid index. Must be between {} and {}", .input, .min, .max)]
    Index {
        input: usize,
        min: usize,
        max: usize,
    },
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

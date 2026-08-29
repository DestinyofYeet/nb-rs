use clap::Subcommand;

use crate::app::args::{ModifyArgs, SyncArgs};

#[derive(Debug, Subcommand)]
pub enum ActionArgs {
    #[command(about = "Create a notebook or a note", visible_aliases=["c"])]
    Create {
        #[arg(help = "The notebook to create")]
        notebook: Option<String>,

        #[arg(help = "The note to create")]
        note: Option<String>,
    },

    #[command(about = "Open a note", visible_aliases=["e", "edit"])]
    Open {
        #[arg(help = "The notebook containing the note")]
        notebook: String,

        #[arg(help = "The filename or title of the note")]
        note: String,
    },

    #[command(about = "List all notes in a notebook or all notebooks", visible_aliases=["ls"])]
    List {
        #[arg(help = "The notebook to list all notes in")]
        notebook: Option<String>,
    },

    #[command(about = "Delete a note or notebook", visible_aliases=["del", "rm"])]
    Delete {
        #[arg(help = "The notebook containing the note")]
        notebook: String,

        #[arg(help = "The filename or title of the note")]
        note: Option<String>,
    },

    #[command(about = "Setup syncing")]
    Sync {
        #[arg(help = "The notebook to setup tracking on")]
        notebook: String,

        #[command(subcommand)]
        action: SyncArgs,
    },

    #[command(about = "Modify notes or notebook")]
    Modify {
        #[arg(help = "The notebook to modify")]
        notebook: String,

        #[command(subcommand)]
        action: ModifyArgs,
    },

    #[command(about = "Search a note in a notebook")]
    Search {
        #[arg(help = "The notebook to search in")]
        notebook: String,

        #[arg(long, help = "The title of the note", group = "name")]
        title: Option<String>,

        #[arg(long, help = "The filename of the note", group = "name")]
        filename: Option<String>,

        #[arg(long, help = "The content of the note", num_args = 1..,)]
        content: Vec<String>,

        #[arg(long, short, help = "Tags to filter", num_args = 1.., )]
        tags: Vec<String>,
    },
}

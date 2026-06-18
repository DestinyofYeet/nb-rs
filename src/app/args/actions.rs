use clap::Subcommand;

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

    #[command(about = "List all notes in a notebook", visible_aliases=["ls"])]
    List {
        #[arg(help = "The notebook to list all notes in")]
        notebook: String,
    }, // #[command(about = "Delete a note", visible_aliases=["del", "rm"])]
       // Delete {
       //     #[arg(help = "The notebook containing the note")]
       //     notebook: String,

       //     #[arg(help = "The filename or title of the note")]
       //     note: String,
       // },
}

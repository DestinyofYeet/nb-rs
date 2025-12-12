use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum ActionArgs {
    #[command(about = "Create a folder or a note")]
    Create {
        #[arg(group = "create", short = 'f', help = "The folder to create")]
        folder: Option<String>,

        #[arg(group = "create", short = 'n', help = "The note to create")]
        note: Option<String>,
    },

    #[command(about = "Open a note")]
    Open {
        #[arg(help = "The note to open")]
        note: String,
    },

    #[command(about = "List items in a folder")]
    Ls {
        #[arg(
            help = "Choose a specific folder",
            default_value = ".",
            required = false
        )]
        folder: String,
    },

    #[command(about = "Remove a folder or note")]
    Rm {
        #[arg(group = "remove", short = 'f', help = "The folder to remove")]
        folder: Option<String>,

        #[arg(group = "remove", short = 'n', help = "The note to remove")]
        note: Option<String>,
    },
}

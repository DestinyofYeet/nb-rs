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
}

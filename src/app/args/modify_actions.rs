use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum ModifyArgs {
    #[command(about = "Modify the title")]
    Title {
        #[arg(help = "The note to modify")]
        note: Option<String>,
    },

    #[command(about = "Modify the tags")]
    Tags {
        #[arg(help = "The filename of the note to modify")]
        note: Option<String>,
    },
}

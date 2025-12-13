use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum SetupSyncArgs {
    Setup {
        #[arg(help = "The repository url to sync with", short = 'r')]
        repo: String,

        #[arg(help = "The branch of the repository to sync with", short = 'b')]
        branch: String,
    },
}

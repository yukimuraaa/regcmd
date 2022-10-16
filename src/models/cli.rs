use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {

    #[clap(subcommand)]
    pub subcommand: SubCommand
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    /// show registered command list
    List,

    // register command
    Register {
        #[clap(short, long)]
        name: Option<String>,

        #[clap(short, long)]
        command: Option<String>,

        #[clap(short, long)]
        description: Option<String>
    },

    /// delete registered command
    Delete {
        #[clap(required = true)]
        name: String
    },
}

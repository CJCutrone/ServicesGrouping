use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(
author="CJ Cutrone III",
version,
about="A simple CLI to automatically assign users to services groups")
]
pub struct Arguments
{
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands
{
    ///load data from file and insert its contents into the database
    Update {
        ///path to the file to load
        path: String
    },
}
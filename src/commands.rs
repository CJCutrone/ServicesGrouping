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
    Api,
    ///load data from file and insert its contents into the database
    Update {
        ///path to the file to load
        path: String
    },
    ///generate group assignments
    Assign {
        ///group id to generate assignments for
        group_id: String,
        ///start date to generate assignments for
        start_date: String,
        ///end date to generate assignments for
        end_date: String,
        ///cron expression to generate assignments for
        cron: String
    }
}
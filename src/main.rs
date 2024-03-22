use clap::Parser;

use crate::actions::data::get_db_connection;
use crate::commands::{Arguments, Commands};

pub mod model;
pub mod actions;
pub mod schema;
pub mod commands;

fn main() {
    let args = Arguments::parse();

    if let Commands::Update { path } = args.command {
        let mut connection = get_db_connection();
        actions::data::process(&*path, &mut connection);
    }
}
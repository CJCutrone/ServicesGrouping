use clap::Parser;
use uuid::Uuid;

use crate::actions::data::get_db_connection;
use crate::actions::data::ticketing::generate_assignments;
use crate::commands::{Arguments, Commands};

pub mod model;
pub mod actions;
pub mod schema;
pub mod commands;

fn main() {
    let mut connection = get_db_connection();
    let users = generate_assignments(Uuid::parse_str("926febd6-7d7e-5d60-bd84-e988bfd94c4e").unwrap(), &mut connection);

    // let args = Arguments::parse();
    //
    // if let Commands::Update { path } = args.command {
    //     let mut connection = get_db_connection();
    //     actions::data::process(&*path, &mut connection);
    // }
}
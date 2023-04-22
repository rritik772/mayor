use clap::Parser;
use dotenv::dotenv;
use std::io;

use config::NewConfig;
use db::Db;

mod config;
mod db;
mod schema;

fn main() -> io::Result<()> {
    dotenv().ok();

    let args = NewConfig::parse();

    let root_path = args.root_path;
    let file = args.file;
    let specifier = args.specifier;
    let mut connection = Db::new();

    if root_path.is_none() && file.is_none() {
        if let Some(config) = NewConfig::fetch_from_database(&mut connection, &specifier) {
            println!("Config found: {}", config.root_path);
        } else {
            println!("Config doesn't exist...")
        }
        return Ok(());
    }

    let new_config = NewConfig {
        specifier,
        root_path,
        file,
    };

    if let Ok(_) = new_config.save_to_database(&mut connection) {
        println!("New Config inserted...");
    } else {
        println!("Cannot insert config");
    }

    Ok(())
}

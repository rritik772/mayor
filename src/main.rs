use clap::Parser;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;
use std::{env, io, path::Path, process::Command};

use config::{Config, NewConfig};
use db::Db;

mod config;
mod db;
mod init;
mod schema;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn handle_config(config: Config) -> io::Result<()> {
    // cd into root_path
    env::set_current_dir(Path::new(&config.root_path)).expect("Cannot cd into root_path");

    // open file in editor
    if let Ok(editor) = env::var("EDITOR") {
        let cmd = format!("{}/{}", config.root_path, config.file);

        Command::new(editor)
            .arg(cmd)
            .status()
            .expect("Cannot execute command");
        return Ok(());
    }
    panic!("Cannot find EDITOR variable from enviroment...");
}

fn main() -> io::Result<()> {
    dotenv().ok();

    let path = init::check_path().expect("_MAYOR_DB variable not found");

    let args = NewConfig::parse();

    let root_path = args.root_path;
    let file = args.file;
    let specifier = args.specifier;
    let mut connection = Db::new(&path);

    // migrations
    connection
        .connection
        .run_pending_migrations(MIGRATIONS)
        .unwrap();

    if root_path.is_none() && file.is_none() {
        if let Some(config) = NewConfig::fetch_from_database(&mut connection, &specifier) {
            handle_config(config).unwrap();
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

    if new_config.save_to_database(&mut connection).is_ok() {
        println!("New Config inserted...");
    } else {
        println!("Cannot insert config");
    }

    Ok(())
}

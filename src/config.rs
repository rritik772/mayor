use clap::Parser;
use diesel::{AsChangeset, Insertable, Queryable};

use crate::db::Db;
use crate::schema::mayor;

#[derive(Queryable, AsChangeset, Clone, Debug)]
#[diesel(table_name = mayor)]
pub struct Config {
    pub id: i32,
    pub specifier: String,
    pub root_path: String,
    pub file: String,
}

#[derive(Parser, Insertable, AsChangeset, Clone, Debug)]
#[diesel(table_name = mayor)]
#[command(author, version, about)]
pub struct NewConfig {
    /// specifer to be call from utility
    pub specifier: String,

    /// path the cd'ed in
    #[arg(short, long)]
    pub root_path: Option<String>,

    /// Name of the file to open in editor
    #[arg(short, long)]
    pub file: Option<String>,
}

impl NewConfig {
    pub fn new(root_path: String, file: String, specifier: String) -> Self {
        Self {
            specifier,
            root_path: Some(root_path),
            file: Some(file),
        }
    }

    pub fn save_to_database(
        &self,
        mut connection: &mut Db,
    ) -> Result<usize, diesel::result::Error> {
        Db::insert_config(&mut connection, self)
    }

    pub fn _update_config(&self, mut _connection: &mut Db) -> Result<usize, diesel::result::Error> {
        unimplemented!();
    }

    pub fn delete_from_database(
        &self,
        mut connection: &mut Db,
    ) -> Result<usize, diesel::result::Error> {
        Db::delete_config(&mut connection, &self.specifier)
    }

    pub fn fetch_from_database(mut connection: &mut Db, specifier: &String) -> Option<Config> {
        let config = Db::fetch_config(&mut connection, &specifier);
        config
    }
}

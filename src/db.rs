use crate::config::{Config, NewConfig};

use diesel::{
    result::Error, sqlite::SqliteConnection, Connection, ExpressionMethods, QueryDsl, RunQueryDsl,
};
use std::env;

use crate::schema::mayor;
use crate::schema::mayor::dsl::*;

pub struct Db {
    pub connection: SqliteConnection,
}

impl Default for Db {
    fn default() -> Self {
        Self::new()
    }
}

impl Db {
    pub fn new() -> Self {
        let database_url =
            env::var("DATABASE_URL").expect("Cannot get Database url in ENV file...");

        let connection = SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Cannot connect to database..."));

        Self { connection }
    }

    pub fn insert_config(&mut self, config: &NewConfig) -> Result<usize, Error> {
        diesel::insert_into(mayor::table)
            .values(config)
            .on_conflict(id)
            .do_update()
            .set(config)
            .execute(&mut self.connection)
    }

    pub fn fetch_config(&mut self, other: &String) -> Option<Config> {
        let result = mayor
            .filter(specifier.eq(other))
            .limit(10)
            .load::<Config>(&mut self.connection)
            .expect("Cannot fetch config...");

        if !result.is_empty() {
            let config = result[0].clone();
            return Some(config);
        }

        None
    }

    pub fn _update_config(&mut self, config: Config) -> Result<usize, Error> {
        diesel::update(mayor.find(config.id))
            .set(config)
            .execute(&mut self.connection)
    }

    pub fn delete_config(&mut self, other: &String) -> Result<usize, Error> {
        diesel::delete(mayor.filter(mayor::specifier.eq(other))).execute(&mut self.connection)
    }
}

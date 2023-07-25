use std::error::Error;

use diesel::{sqlite::Sqlite, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn run_migrations(conn: &mut SqliteConnection) {

    conn.run_pending_migrations(MIGRATIONS).unwrap();

}

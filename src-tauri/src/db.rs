use std::{
    fs,
    path::{Path, PathBuf},
};

use sqlx::{sqlite::SqliteConnectOptions, Pool, Sqlite, SqlitePool};

const DB_NAME: &str = "waterer.db";

pub struct Database {
    pub pool: Pool<Sqlite>,
}

pub struct DatabaseState(pub Pool<Sqlite>);

impl Database {
    pub async fn new() -> Result<Database, sqlx::Error> {
        let db_path = get_db_file_path();
        let connection_options = SqliteConnectOptions::new()
            .filename(db_path)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);

        let pool = SqlitePool::connect_with(connection_options).await?;

        // Run migrations regardless of whether the database is new
        // SQLx will track which migrations have been run
        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Self { pool })
    }
}

pub fn init_db() {
    let db_file_path = get_db_file_path();

    if !db_file_path.exists() {
        create_db_file();
    }
}

fn create_db_file() {
    let db_file_path = get_db_file_path();

    let db_directory = db_file_path.parent().unwrap();

    if !db_directory.exists() {
        fs::create_dir(db_directory).unwrap();
    }
    fs::File::create_new(db_file_path).unwrap();
}

fn get_db_file_path() -> PathBuf {
    match dirs::config_local_dir() {
        Some(dir) => dir.join("waterer").join(DB_NAME),
        None => Path::new(".").join("waterer").join(DB_NAME).to_path_buf(),
    }
}

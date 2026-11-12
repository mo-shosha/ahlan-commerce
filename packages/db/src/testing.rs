
use std::time::Duration;

use db_generated::tokio_postgres::{self, Client, NoTls};
use uuid::Uuid;

use crate::{Pool, PoolSettings, pool::connect_with};

const DATABASE_URL_KEY: &str = "DATABASE_URL";
const MIGRATIONS_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../db/migrations");
const TEST_POOL: PoolSettings = PoolSettings { max_size: 5, timeout: Duration::from_secs(30) };

pub struct TestDb {
    database_url: String,
    schema: String,
}

impl TestDb {
    pub async fn create() -> Self {
        let database_url = std::env::var(DATABASE_URL_KEY).unwrap_or_else(|_| {
            panic!("set {DATABASE_URL_KEY} to run tests against PostgreSQL")
        });
        let test_db = Self { database_url, schema: format!("test_{}", Uuid::now_v7().simple()) };

        let admin = test_db.admin_client().await;
        admin
            .batch_execute(&format!("create schema {}", test_db.schema))
            .await
            .expect("create test schema");
        admin
            .batch_execute(&format!("set search_path to {}", test_db.schema))
            .await
            .expect("switch to test schema");
        for (name, sql) in migration_files() {
            admin
                .batch_execute(&sql)
                .await
                .unwrap_or_else(|err| panic!("apply {name}: {err}"));
        }

        test_db
    }

    pub fn database_url(&self) -> &str {
        &self.database_url
    }

    pub async fn pool(&self) -> Pool {
        let mut config: tokio_postgres::Config =
            self.database_url.parse().expect("valid database url");
        config.options(format!("-c search_path={}", self.schema));

        connect_with(config, TEST_POOL).await.expect("connect to test schema")
    }

    pub async fn drop(self) {
        self.admin_client()
            .await
            .batch_execute(&format!("drop schema {} cascade", self.schema))
            .await
            .expect("drop test schema");
    }

    async fn admin_client(&self) -> Client {
        let (client, connection) = tokio_postgres::connect(&self.database_url, NoTls)
            .await
            .expect("connect to PostgreSQL");
        tokio::spawn(connection);
        client
    }
}

fn migration_files() -> Vec<(String, String)> {
    let mut paths: Vec<_> = std::fs::read_dir(MIGRATIONS_DIR)
        .expect("read db/migrations")
        .map(|entry| entry.expect("read migration entry").path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "sql"))
        .collect();
    paths.sort();
    assert!(!paths.is_empty(), "no migrations in db/migrations; run atlas migrate diff");

    paths
        .into_iter()
        .map(|path| {
            let name = path.file_name().unwrap().to_string_lossy().into_owned();
            let sql = std::fs::read_to_string(&path).expect("read migration file");
            (name, sql)
        })
        .collect()
}

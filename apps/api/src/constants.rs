pub mod routes {
    pub const ROOT: &str = "/";
    pub const HEALTH: &str = "/health";
    pub const PRODUCTS: &str = "/api/products";
    pub const SIMULATED_INTERNAL_FAILURE: &str = "/api/debug/internal-failure";
}

pub mod env_keys {
    pub const API_ADDR: &str = "AHLAN_API_ADDR";
    pub const DATABASE_URL: &str = "DATABASE_URL";
}

pub mod defaults {
    use std::time::Duration;

    pub const API_ADDR: &str = "0.0.0.0:3000";
    pub const DATABASE_URL_EXAMPLE: &str = "postgres://postgres:postgres@localhost:5433/ahlan_commerce";
    pub const DB_MAX_CONNECTIONS: u32 = 5;
    pub const DB_ACQUIRE_TIMEOUT: Duration = Duration::from_secs(3);
}

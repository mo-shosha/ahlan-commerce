pub mod routes {
    pub const ROOT: &str = "/";
    pub const HEALTH: &str = "/health";
    pub const PRODUCTS: &str = "/api/products";
}

pub mod env_keys {
    pub const API_ADDR: &str = "AHLAN_API_ADDR";
}

pub mod defaults {
    pub const API_ADDR: &str = "0.0.0.0:3000";
}

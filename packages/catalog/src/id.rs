use std::fmt;

use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProductId(Uuid);

impl ProductId {
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl fmt::Display for ProductId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImportJobId(Uuid);

impl ImportJobId {
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl fmt::Display for ImportJobId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub trait IdGenerator: Send + Sync {
    fn new_product_id(&self) -> ProductId;
    fn new_import_job_id(&self) -> ImportJobId;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct UuidV7Generator;

impl IdGenerator for UuidV7Generator {
    fn new_product_id(&self) -> ProductId {
        ProductId(Uuid::now_v7())
    }

    fn new_import_job_id(&self) -> ImportJobId {
        ImportJobId(Uuid::now_v7())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uuid_v7_generator_produces_version_7_ids() {
        let id = UuidV7Generator.new_product_id();

        assert_eq!(id.as_uuid().get_version_num(), 7);
    }

    #[test]
    fn uuid_v7_generator_produces_unique_ids() {
        let first = UuidV7Generator.new_product_id();
        let second = UuidV7Generator.new_product_id();

        assert_ne!(first, second);
    }
}

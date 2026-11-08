//! Deterministic ID and clock implementations for tests.

use std::collections::VecDeque;
use std::sync::Mutex;

use uuid::Uuid;

use crate::{Clock, IdGenerator, ProductId, Timestamp};

pub struct FixedIds {
    remaining: Mutex<VecDeque<ProductId>>,
}

impl FixedIds {

    pub fn new<'a>(ids: impl IntoIterator<Item = &'a str>) -> Self {
        let remaining = ids
            .into_iter()
            .map(|raw| {
                let uuid = Uuid::parse_str(raw)
                    .unwrap_or_else(|err| panic!("invalid fixed id {raw:?}: {err}"));
                assert_eq!(uuid.get_version_num(), 7, "fixed id {raw:?} is not a UUIDv7");
                ProductId::from_uuid(uuid)
            })
            .collect();

        Self {
            remaining: Mutex::new(remaining),
        }
    }
}

impl IdGenerator for FixedIds {
    fn new_product_id(&self) -> ProductId {
        self.remaining
            .lock()
            .expect("fixed id lock poisoned")
            .pop_front()
            .expect("FixedIds ran out of ids")
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FixedClock(Timestamp);

impl FixedClock {
    pub fn new(at: Timestamp) -> Self {
        Self(at)
    }

    pub fn from_rfc3339(raw: &str) -> Self {
        let at = raw
            .parse()
            .unwrap_or_else(|err| panic!("invalid fixed time {raw:?}: {err}"));
        Self(at)
    }
}

impl Clock for FixedClock {
    fn now(&self) -> Timestamp {
        self.0
    }
}

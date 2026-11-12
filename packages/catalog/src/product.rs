use crate::{ProductId, Timestamp};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewProduct {
    pub title: String,
    pub handle: String,
    pub description: Option<String>,
    pub price_cents: u32,
    pub inventory_quantity: u32,
    pub published: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Product {
    pub id: ProductId,
    pub title: String,
    pub handle: String,
    pub description: Option<String>,
    pub price_cents: u32,
    pub inventory_quantity: u32,
    pub published: bool,
    pub published_at: Option<Timestamp>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl Product {
    pub(crate) fn create(id: ProductId, input: NewProduct, now: Timestamp) -> Self {
        Self {
            id,
            title: input.title,
            handle: input.handle,
            description: input.description,
            price_cents: input.price_cents,
            inventory_quantity: input.inventory_quantity,
            published: input.published,
            published_at: published_at(input.published, now),
            created_at: now,
            updated_at: now,
        }
    }
}

pub(crate) fn published_at(published: bool, now: Timestamp) -> Option<Timestamp> {
    published.then_some(now)
}

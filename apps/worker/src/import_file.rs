use catalog::NewProduct;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ImportFile {
    pub products: Vec<ImportProduct>,
}

#[derive(Debug, Deserialize)]
pub struct ImportProduct {
    pub title: String,
    pub handle: String,
    pub description: Option<String>,
    pub price_cents: u32,
    pub inventory_quantity: u32,
    pub published: bool,
}

impl From<ImportProduct> for NewProduct {
    fn from(product: ImportProduct) -> Self {
        Self {
            title: product.title,
            handle: product.handle,
            description: product.description,
            price_cents: product.price_cents,
            inventory_quantity: product.inventory_quantity,
            published: product.published,
        }
    }
}

pub fn parse_import_file(bytes: &[u8]) -> Result<ImportFile, String> {
    serde_json::from_slice(bytes).map_err(|_| {
        "Import file is not valid JSON for the product import contract.".to_string()
    })
}

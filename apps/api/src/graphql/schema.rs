use std::sync::Arc;

use async_graphql::{
    Context, EmptySubscription, ID, InputObject, Object, Result, Schema, SimpleObject,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::State;
use axum::Extension;
use catalog::{NewProduct, ProductCatalog};
use db::Pool;

use crate::graphql::datetime::DateTime;
use crate::graphql::error;
use crate::state::AppState;

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;


#[derive(Clone)]
pub struct GraphqlContext {
    pub db: Pool,
    pub catalog: Arc<ProductCatalog>,
}

#[derive(Clone, Debug)]
pub struct RequestId(pub String);

#[derive(SimpleObject)]
pub struct Product {
    pub id: ID,
    pub title: String,
    pub handle: String,
    pub description: Option<String>,
    pub price_cents: i32,
    pub inventory_quantity: i32,
    pub published: bool,
    pub published_at: Option<DateTime>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl From<catalog::Product> for Product {
    fn from(product: catalog::Product) -> Self {
        Self {
            id: ID(product.id.to_string()),
            title: product.title,
            handle: product.handle,
            description: product.description,
            price_cents: i32::try_from(product.price_cents)
                .expect("persisted price_cents fits in GraphQL Int"),
            inventory_quantity: i32::try_from(product.inventory_quantity)
                .expect("persisted inventory_quantity fits in GraphQL Int"),
            published: product.published,
            published_at: product.published_at.map(DateTime::from),
            created_at: DateTime::from(product.created_at),
            updated_at: DateTime::from(product.updated_at),
        }
    }
}

#[derive(InputObject)]
pub struct ProductCreateInput {
    pub title: String,
    pub handle: String,
    pub description: Option<String>,
    pub price_cents: i32,
    pub inventory_quantity: i32,
    pub published: bool,
}

impl ProductCreateInput {
    fn into_new_product(self, request_id: &str) -> Result<NewProduct> {
        let price_cents = u32::try_from(self.price_cents).map_err(|_| {
            error::validation("priceCents must be at least 0.", request_id)
        })?;
        let inventory_quantity = u32::try_from(self.inventory_quantity).map_err(|_| {
            error::validation("inventoryQuantity must be at least 0.", request_id)
        })?;

        Ok(NewProduct {
            title: self.title,
            handle: self.handle,
            description: self.description,
            price_cents,
            inventory_quantity,
            published: self.published,
        })
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn products(&self, ctx: &Context<'_>) -> Result<Vec<Product>> {
        let gql = ctx.data::<GraphqlContext>()?;
        let request_id = request_id(ctx)?;

        let products = gql
            .catalog
            .list_products(&gql.db)
            .await
            .map_err(|err| error::from_catalog(err, request_id))?;

        Ok(products.into_iter().map(Product::from).collect())
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn product_create(
        &self,
        ctx: &Context<'_>,
        input: ProductCreateInput,
    ) -> Result<Product> {
        let gql = ctx.data::<GraphqlContext>()?;
        let request_id = request_id(ctx)?;
        let product_handle = input.handle.clone();
        let new_product = input.into_new_product(request_id)?;

        let product = gql
            .catalog
            .create_product(&gql.db, new_product)
            .await
            .map_err(|err| {
                let gql_error = error::from_catalog(err, request_id);
                tracing::warn!(
                    operation = "product_create",
                    product_handle = %product_handle,
                    "Product creation rejected via GraphQL"
                );
                gql_error
            })?;

        tracing::info!(
            operation = "product_create",
            product_id = %product.id,
            product_handle = %product.handle,
            "Product created via GraphQL"
        );

        Ok(Product::from(product))
    }
}

pub fn build_schema(ctx: GraphqlContext) -> AppSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(ctx)
        .finish()
}

pub async fn graphql_handler(
    State(state): State<AppState>,
    Extension(request_id): Extension<String>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    state
        .schema
        .execute(request.into_inner().data(RequestId(request_id)))
        .await
        .into()
}

fn request_id<'a>(ctx: &'a Context<'_>) -> Result<&'a str> {
    Ok(&ctx.data::<RequestId>()?.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_sdl_matches_chapter_contract() {
        let sdl = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
            .finish()
            .sdl();

        assert!(sdl.contains("scalar DateTime"), "{sdl}");
        assert!(sdl.contains("type Product"), "{sdl}");
        assert!(sdl.contains("priceCents: Int!"), "{sdl}");
        assert!(sdl.contains("inventoryQuantity: Int!"), "{sdl}");
        assert!(sdl.contains("publishedAt: DateTime"), "{sdl}");
        assert!(sdl.contains("createdAt: DateTime!"), "{sdl}");
        assert!(sdl.contains("updatedAt: DateTime!"), "{sdl}");
        assert!(sdl.contains("products: [Product!]!"), "{sdl}");
        assert!(
            sdl.contains("productCreate(input: ProductCreateInput!): Product!"),
            "{sdl}"
        );
        assert!(sdl.contains("input ProductCreateInput"), "{sdl}");
    }
}

// This file was generated with `cornucopia`. Do not modify.

#[derive(Debug)]
pub struct CreateProductParams<T1: crate::StringSql, T2: crate::StringSql, T3: crate::StringSql> {
    pub id: uuid::Uuid,
    pub title: T1,
    pub handle: T2,
    pub description: Option<T3>,
    pub price_cents: i32,
    pub inventory_quantity: i32,
    pub published: bool,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct CreateProduct {
    pub id: uuid::Uuid,
    pub title: String,
    pub handle: String,
    pub description: Option<String>,
    pub price_cents: i32,
    pub inventory_quantity: i32,
    pub published: bool,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
pub struct CreateProductBorrowed<'a> {
    pub id: uuid::Uuid,
    pub title: &'a str,
    pub handle: &'a str,
    pub description: Option<&'a str>,
    pub price_cents: i32,
    pub inventory_quantity: i32,
    pub published: bool,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
impl<'a> From<CreateProductBorrowed<'a>> for CreateProduct {
    fn from(
        CreateProductBorrowed {
            id,
            title,
            handle,
            description,
            price_cents,
            inventory_quantity,
            published,
            published_at,
            created_at,
            updated_at,
        }: CreateProductBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            title: title.into(),
            handle: handle.into(),
            description: description.map(|v| v.into()),
            price_cents,
            inventory_quantity,
            published,
            published_at,
            created_at,
            updated_at,
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct CreateProductQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<CreateProductBorrowed, tokio_postgres::Error>,
    mapper: fn(CreateProductBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> CreateProductQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(CreateProductBorrowed) -> R,
    ) -> CreateProductQuery<'c, 'a, 's, C, R, N> {
        CreateProductQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct CreateProductStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create_product() -> CreateProductStmt {
    CreateProductStmt(
        "insert into products (id, title, handle, description, price_cents, inventory_quantity, published, published_at, created_at, updated_at) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) returning id, title, handle, description, price_cents, inventory_quantity, published, published_at, created_at, updated_at",
        None,
    )
}
impl CreateProductStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
    >(
        &'s self,
        client: &'c C,
        id: &'a uuid::Uuid,
        title: &'a T1,
        handle: &'a T2,
        description: &'a Option<T3>,
        price_cents: &'a i32,
        inventory_quantity: &'a i32,
        published: &'a bool,
        published_at: &'a Option<chrono::DateTime<chrono::Utc>>,
        created_at: &'a chrono::DateTime<chrono::Utc>,
        updated_at: &'a chrono::DateTime<chrono::Utc>,
    ) -> CreateProductQuery<'c, 'a, 's, C, CreateProduct, 10> {
        CreateProductQuery {
            client,
            params: [
                id,
                title,
                handle,
                description,
                price_cents,
                inventory_quantity,
                published,
                published_at,
                created_at,
                updated_at,
            ],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<CreateProductBorrowed, tokio_postgres::Error> {
                    Ok(CreateProductBorrowed {
                        id: row.try_get(0)?,
                        title: row.try_get(1)?,
                        handle: row.try_get(2)?,
                        description: row.try_get(3)?,
                        price_cents: row.try_get(4)?,
                        inventory_quantity: row.try_get(5)?,
                        published: row.try_get(6)?,
                        published_at: row.try_get(7)?,
                        created_at: row.try_get(8)?,
                        updated_at: row.try_get(9)?,
                    })
                },
            mapper: |it| CreateProduct::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql, T3: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        CreateProductParams<T1, T2, T3>,
        CreateProductQuery<'c, 'a, 's, C, CreateProduct, 10>,
        C,
    > for CreateProductStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a CreateProductParams<T1, T2, T3>,
    ) -> CreateProductQuery<'c, 'a, 's, C, CreateProduct, 10> {
        self.bind(
            client,
            &params.id,
            &params.title,
            &params.handle,
            &params.description,
            &params.price_cents,
            &params.inventory_quantity,
            &params.published,
            &params.published_at,
            &params.created_at,
            &params.updated_at,
        )
    }
}

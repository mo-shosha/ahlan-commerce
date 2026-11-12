// This file was generated with `cornucopia`. Do not modify.

#[derive(Debug, Clone, PartialEq)]
pub struct ListProducts {
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
pub struct ListProductsBorrowed<'a> {
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
impl<'a> From<ListProductsBorrowed<'a>> for ListProducts {
    fn from(
        ListProductsBorrowed {
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
        }: ListProductsBorrowed<'a>,
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
pub struct ListProductsQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ListProductsBorrowed, tokio_postgres::Error>,
    mapper: fn(ListProductsBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ListProductsQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ListProductsBorrowed) -> R,
    ) -> ListProductsQuery<'c, 'a, 's, C, R, N> {
        ListProductsQuery {
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
pub struct ListProductsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn list_products() -> ListProductsStmt {
    ListProductsStmt(
        "select id, title, handle, description, price_cents, inventory_quantity, published, published_at, created_at, updated_at from products order by created_at asc, id asc",
        None,
    )
}
impl ListProductsStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
    ) -> ListProductsQuery<'c, 'a, 's, C, ListProducts, 0> {
        ListProductsQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<ListProductsBorrowed, tokio_postgres::Error> {
                    Ok(ListProductsBorrowed {
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
            mapper: |it| ListProducts::from(it),
        }
    }
}

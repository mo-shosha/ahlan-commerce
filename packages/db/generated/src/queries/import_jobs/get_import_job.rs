// This file was generated with `cornucopia`. Do not modify.

#[derive(Debug, Clone, PartialEq)]
pub struct GetImportJob {
    pub id: uuid::Uuid,
    pub status: String,
    pub input_path: String,
    pub attempts: i32,
    pub last_error: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
pub struct GetImportJobBorrowed<'a> {
    pub id: uuid::Uuid,
    pub status: &'a str,
    pub input_path: &'a str,
    pub attempts: i32,
    pub last_error: Option<&'a str>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
impl<'a> From<GetImportJobBorrowed<'a>> for GetImportJob {
    fn from(
        GetImportJobBorrowed {
            id,
            status,
            input_path,
            attempts,
            last_error,
            created_at,
            updated_at,
        }: GetImportJobBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            status: status.into(),
            input_path: input_path.into(),
            attempts,
            last_error: last_error.map(|v| v.into()),
            created_at,
            updated_at,
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct GetImportJobQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<GetImportJobBorrowed, tokio_postgres::Error>,
    mapper: fn(GetImportJobBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> GetImportJobQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(GetImportJobBorrowed) -> R,
    ) -> GetImportJobQuery<'c, 'a, 's, C, R, N> {
        GetImportJobQuery {
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
pub struct GetImportJobStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_import_job() -> GetImportJobStmt {
    GetImportJobStmt(
        "select id, status, input_path, attempts, last_error, created_at, updated_at from import_jobs where id = $1",
        None,
    )
}
impl GetImportJobStmt {
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
        id: &'a uuid::Uuid,
    ) -> GetImportJobQuery<'c, 'a, 's, C, GetImportJob, 1> {
        GetImportJobQuery {
            client,
            params: [id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<GetImportJobBorrowed, tokio_postgres::Error> {
                    Ok(GetImportJobBorrowed {
                        id: row.try_get(0)?,
                        status: row.try_get(1)?,
                        input_path: row.try_get(2)?,
                        attempts: row.try_get(3)?,
                        last_error: row.try_get(4)?,
                        created_at: row.try_get(5)?,
                        updated_at: row.try_get(6)?,
                    })
                },
            mapper: |it| GetImportJob::from(it),
        }
    }
}

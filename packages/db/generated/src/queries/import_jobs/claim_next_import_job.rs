// This file was generated with `cornucopia`. Do not modify.

#[derive(Clone, Copy, Debug)]
pub struct ClaimNextImportJobParams {
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub max_attempts: i32,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ClaimNextImportJob {
    pub id: uuid::Uuid,
    pub status: String,
    pub input_path: String,
    pub attempts: i32,
    pub last_error: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
pub struct ClaimNextImportJobBorrowed<'a> {
    pub id: uuid::Uuid,
    pub status: &'a str,
    pub input_path: &'a str,
    pub attempts: i32,
    pub last_error: Option<&'a str>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
impl<'a> From<ClaimNextImportJobBorrowed<'a>> for ClaimNextImportJob {
    fn from(
        ClaimNextImportJobBorrowed {
            id,
            status,
            input_path,
            attempts,
            last_error,
            created_at,
            updated_at,
        }: ClaimNextImportJobBorrowed<'a>,
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
pub struct ClaimNextImportJobQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor:
        fn(&tokio_postgres::Row) -> Result<ClaimNextImportJobBorrowed, tokio_postgres::Error>,
    mapper: fn(ClaimNextImportJobBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ClaimNextImportJobQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(ClaimNextImportJobBorrowed) -> R,
    ) -> ClaimNextImportJobQuery<'c, 'a, 's, C, R, N> {
        ClaimNextImportJobQuery {
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
pub struct ClaimNextImportJobStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn claim_next_import_job() -> ClaimNextImportJobStmt {
    ClaimNextImportJobStmt(
        "update import_jobs set status = 'running', attempts = attempts + 1, updated_at = $1 where id = ( select id from import_jobs where status = 'queued' and attempts < $2 order by created_at asc, id asc limit 1 ) returning id, status, input_path, attempts, last_error, created_at, updated_at",
        None,
    )
}
impl ClaimNextImportJobStmt {
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
        updated_at: &'a chrono::DateTime<chrono::Utc>,
        max_attempts: &'a i32,
    ) -> ClaimNextImportJobQuery<'c, 'a, 's, C, ClaimNextImportJob, 2> {
        ClaimNextImportJobQuery {
            client,
            params: [updated_at, max_attempts],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<ClaimNextImportJobBorrowed, tokio_postgres::Error> {
                Ok(ClaimNextImportJobBorrowed {
                    id: row.try_get(0)?,
                    status: row.try_get(1)?,
                    input_path: row.try_get(2)?,
                    attempts: row.try_get(3)?,
                    last_error: row.try_get(4)?,
                    created_at: row.try_get(5)?,
                    updated_at: row.try_get(6)?,
                })
            },
            mapper: |it| ClaimNextImportJob::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        ClaimNextImportJobParams,
        ClaimNextImportJobQuery<'c, 'a, 's, C, ClaimNextImportJob, 2>,
        C,
    > for ClaimNextImportJobStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a ClaimNextImportJobParams,
    ) -> ClaimNextImportJobQuery<'c, 'a, 's, C, ClaimNextImportJob, 2> {
        self.bind(client, &params.updated_at, &params.max_attempts)
    }
}

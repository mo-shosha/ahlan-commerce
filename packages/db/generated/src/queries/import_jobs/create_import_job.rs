// This file was generated with `cornucopia`. Do not modify.

#[derive(Debug)]
pub struct CreateImportJobParams<T1: crate::StringSql, T2: crate::StringSql, T3: crate::StringSql> {
    pub id: uuid::Uuid,
    pub status: T1,
    pub input_path: T2,
    pub attempts: i32,
    pub last_error: Option<T3>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct CreateImportJob {
    pub id: uuid::Uuid,
    pub status: String,
    pub input_path: String,
    pub attempts: i32,
    pub last_error: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
pub struct CreateImportJobBorrowed<'a> {
    pub id: uuid::Uuid,
    pub status: &'a str,
    pub input_path: &'a str,
    pub attempts: i32,
    pub last_error: Option<&'a str>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
impl<'a> From<CreateImportJobBorrowed<'a>> for CreateImportJob {
    fn from(
        CreateImportJobBorrowed {
            id,
            status,
            input_path,
            attempts,
            last_error,
            created_at,
            updated_at,
        }: CreateImportJobBorrowed<'a>,
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
pub struct CreateImportJobQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<CreateImportJobBorrowed, tokio_postgres::Error>,
    mapper: fn(CreateImportJobBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> CreateImportJobQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(CreateImportJobBorrowed) -> R,
    ) -> CreateImportJobQuery<'c, 'a, 's, C, R, N> {
        CreateImportJobQuery {
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
pub struct CreateImportJobStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create_import_job() -> CreateImportJobStmt {
    CreateImportJobStmt(
        "insert into import_jobs (id, status, input_path, attempts, last_error, created_at, updated_at) values ($1, $2, $3, $4, $5, $6, $7) returning id, status, input_path, attempts, last_error, created_at, updated_at",
        None,
    )
}
impl CreateImportJobStmt {
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
        status: &'a T1,
        input_path: &'a T2,
        attempts: &'a i32,
        last_error: &'a Option<T3>,
        created_at: &'a chrono::DateTime<chrono::Utc>,
        updated_at: &'a chrono::DateTime<chrono::Utc>,
    ) -> CreateImportJobQuery<'c, 'a, 's, C, CreateImportJob, 7> {
        CreateImportJobQuery {
            client,
            params: [
                id,
                status,
                input_path,
                attempts,
                last_error,
                created_at,
                updated_at,
            ],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<CreateImportJobBorrowed, tokio_postgres::Error> {
                Ok(CreateImportJobBorrowed {
                    id: row.try_get(0)?,
                    status: row.try_get(1)?,
                    input_path: row.try_get(2)?,
                    attempts: row.try_get(3)?,
                    last_error: row.try_get(4)?,
                    created_at: row.try_get(5)?,
                    updated_at: row.try_get(6)?,
                })
            },
            mapper: |it| CreateImportJob::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql, T3: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        CreateImportJobParams<T1, T2, T3>,
        CreateImportJobQuery<'c, 'a, 's, C, CreateImportJob, 7>,
        C,
    > for CreateImportJobStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a CreateImportJobParams<T1, T2, T3>,
    ) -> CreateImportJobQuery<'c, 'a, 's, C, CreateImportJob, 7> {
        self.bind(
            client,
            &params.id,
            &params.status,
            &params.input_path,
            &params.attempts,
            &params.last_error,
            &params.created_at,
            &params.updated_at,
        )
    }
}

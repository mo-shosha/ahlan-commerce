use chrono::{DateTime, Utc};
use db_generated::client::Params;
use db_generated::queries::import_jobs::{
    claim_next_import_job, complete_import_job, create_import_job, fail_import_job, get_import_job,
    requeue_import_job,
};
use uuid::Uuid;

use crate::{DbError, Pool};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImportJobRecord {
    pub id: Uuid,
    pub status: String,
    pub input_path: String,
    pub attempts: i32,
    pub last_error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn create_import_job(db: &Pool, job: &ImportJobRecord) -> Result<ImportJobRecord, DbError> {
    let client = db.get().await?;

    let row = create_import_job::create_import_job()
        .params(
            &client,
            &create_import_job::CreateImportJobParams {
                id: job.id,
                status: &job.status,
                input_path: &job.input_path,
                attempts: job.attempts,
                last_error: job.last_error.as_deref(),
                created_at: job.created_at,
                updated_at: job.updated_at,
            },
        )
        .one()
        .await?;

    Ok(row.into())
}

pub async fn claim_next_import_job(
    db: &Pool,
    updated_at: DateTime<Utc>,
    max_attempts: i32,
) -> Result<Option<ImportJobRecord>, DbError> {
    let client = db.get().await?;

    let row = claim_next_import_job::claim_next_import_job()
        .params(
            &client,
            &claim_next_import_job::ClaimNextImportJobParams {
                updated_at,
                max_attempts,
            },
        )
        .opt()
        .await?;

    Ok(row.map(ImportJobRecord::from))
}

pub async fn complete_import_job(
    db: &Pool,
    id: Uuid,
    updated_at: DateTime<Utc>,
) -> Result<Option<ImportJobRecord>, DbError> {
    let client = db.get().await?;

    let row = complete_import_job::complete_import_job()
        .params(
            &client,
            &complete_import_job::CompleteImportJobParams { id, updated_at },
        )
        .opt()
        .await?;

    Ok(row.map(ImportJobRecord::from))
}

pub async fn fail_import_job(
    db: &Pool,
    id: Uuid,
    last_error: &str,
    updated_at: DateTime<Utc>,
) -> Result<Option<ImportJobRecord>, DbError> {
    let client = db.get().await?;

    let row = fail_import_job::fail_import_job()
        .params(
            &client,
            &fail_import_job::FailImportJobParams {
                id,
                last_error: Some(last_error),
                updated_at,
            },
        )
        .opt()
        .await?;

    Ok(row.map(ImportJobRecord::from))
}

pub async fn get_import_job(db: &Pool, id: Uuid) -> Result<Option<ImportJobRecord>, DbError> {
    let client = db.get().await?;

    let row = get_import_job::get_import_job()
        .bind(&client, &id)
        .opt()
        .await?;

    Ok(row.map(ImportJobRecord::from))
}

pub async fn requeue_import_job(
    db: &Pool,
    id: Uuid,
    updated_at: DateTime<Utc>,
) -> Result<Option<ImportJobRecord>, DbError> {
    let client = db.get().await?;

    let row = requeue_import_job::requeue_import_job()
        .params(
            &client,
            &requeue_import_job::RequeueImportJobParams { id, updated_at },
        )
        .opt()
        .await?;

    Ok(row.map(ImportJobRecord::from))
}

macro_rules! import_job_record_from_rows {
    ($($row:ty),+ $(,)?) => {$(
        impl From<$row> for ImportJobRecord {
            fn from(row: $row) -> Self {
                Self {
                    id: row.id,
                    status: row.status,
                    input_path: row.input_path,
                    attempts: row.attempts,
                    last_error: row.last_error,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                }
            }
        }
    )+};
}

import_job_record_from_rows!(
    create_import_job::CreateImportJob,
    claim_next_import_job::ClaimNextImportJob,
    complete_import_job::CompleteImportJob,
    fail_import_job::FailImportJob,
    get_import_job::GetImportJob,
    requeue_import_job::RequeueImportJob,
);

use std::sync::Arc;

use db::Pool;
use db::import_jobs::{self, ImportJobRecord};

use crate::import_job::{ImportJob, MAX_IMPORT_ATTEMPTS};
use crate::{CatalogError, Clock, IdGenerator, ImportJobId};

#[derive(Clone)]
pub struct ImportJobService {
    ids: Arc<dyn IdGenerator>,
    clock: Arc<dyn Clock>,
}

impl ImportJobService {
    pub fn new(ids: Arc<dyn IdGenerator>, clock: Arc<dyn Clock>) -> Self {
        Self { ids, clock }
    }

    pub async fn enqueue(&self, db: &Pool, input_path: String) -> Result<ImportJob, CatalogError> {
        let input_path = input_path.trim().to_string();
        if input_path.is_empty() {
            return Err(CatalogError::EmptyImportInputPath);
        }

        let job = ImportJob::queued(self.ids.new_import_job_id(), input_path, self.clock.now());
        let created = import_jobs::create_import_job(db, &to_record(&job)).await?;
        Ok(from_record(created))
    }

    pub async fn claim_next(&self, db: &Pool) -> Result<Option<ImportJob>, CatalogError> {
        let claimed = import_jobs::claim_next_import_job(db, self.clock.now(), MAX_IMPORT_ATTEMPTS)
            .await?;
        Ok(claimed.map(from_record))
    }

    pub async fn mark_succeeded(&self, db: &Pool, job_id: ImportJobId) -> Result<ImportJob, CatalogError> {
        let updated = import_jobs::complete_import_job(db, job_id.as_uuid(), self.clock.now())
            .await?
            .ok_or(CatalogError::ImportJobNotFound)?;
        Ok(from_record(updated))
    }

    pub async fn mark_failed(
        &self,
        db: &Pool,
        job_id: ImportJobId,
        last_error: String,
    ) -> Result<ImportJob, CatalogError> {
        let updated =
            import_jobs::fail_import_job(db, job_id.as_uuid(), &last_error, self.clock.now())
                .await?
                .ok_or(CatalogError::ImportJobNotFound)?;
        Ok(from_record(updated))
    }

    pub async fn get(&self, db: &Pool, job_id: ImportJobId) -> Result<ImportJob, CatalogError> {
        let record = import_jobs::get_import_job(db, job_id.as_uuid())
            .await?
            .ok_or(CatalogError::ImportJobNotFound)?;
        Ok(from_record(record))
    }

    /// `failed` -> `queued` without clearing `attempts` (Chapter 11 retry rule).
    pub async fn requeue(&self, db: &Pool, job_id: ImportJobId) -> Result<ImportJob, CatalogError> {
        let updated = import_jobs::requeue_import_job(db, job_id.as_uuid(), self.clock.now())
            .await?
            .ok_or(CatalogError::ImportJobNotFound)?;
        Ok(from_record(updated))
    }
}

fn to_record(job: &ImportJob) -> ImportJobRecord {
    ImportJobRecord {
        id: job.id.as_uuid(),
        status: job.status.clone(),
        input_path: job.input_path.clone(),
        attempts: job.attempts,
        last_error: job.last_error.clone(),
        created_at: job.created_at,
        updated_at: job.updated_at,
    }
}

fn from_record(record: ImportJobRecord) -> ImportJob {
    ImportJob {
        id: ImportJobId::from_uuid(record.id),
        status: record.status,
        input_path: record.input_path,
        attempts: record.attempts,
        last_error: record.last_error,
        created_at: record.created_at,
        updated_at: record.updated_at,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::import_job::STATUS_QUEUED;
    use crate::testing::{FixedClock, FixedIds};

    const JOB_ID: &str = "018f7b2a-9f62-7d0c-8c4f-7a1f37c1d010";
    const NOW: &str = "2026-01-01T00:00:00Z";

    fn service() -> ImportJobService {
        ImportJobService::new(
            Arc::new(FixedIds::new([JOB_ID])),
            Arc::new(FixedClock::from_rfc3339(NOW)),
        )
    }

    #[test]
    fn queued_job_starts_with_zero_attempts() {
        let job = ImportJob::queued(
            ImportJobId::from_uuid(uuid::Uuid::parse_str(JOB_ID).unwrap()),
            "fixtures/products-import.json".to_string(),
            FixedClock::from_rfc3339(NOW).now(),
        );

        assert_eq!(job.status, STATUS_QUEUED);
        assert_eq!(job.attempts, 0);
        assert_eq!(job.last_error, None);
        let _service = service();
    }
}

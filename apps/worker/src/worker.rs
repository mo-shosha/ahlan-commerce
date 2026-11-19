use std::time::Duration;

use catalog::{
    CatalogError, ImportJob, ImportJobService, ProductCatalog, STATUS_FAILED, STATUS_RUNNING,
    STATUS_SUCCEEDED,
};
use db::Pool;
use tokio::fs;

use crate::import_file::{parse_import_file, ImportProduct};

pub struct Worker {
    db: Pool,
    catalog: ProductCatalog,
    import_jobs: ImportJobService,
}

impl Worker {
    pub fn new(db: Pool, catalog: ProductCatalog, import_jobs: ImportJobService) -> Self {
        Self {
            db,
            catalog,
            import_jobs,
        }
    }

    pub async fn run(&self, poll_interval: Duration) {
        tracing::info!("Import worker started");

        loop {
            match self.tick().await {
                Ok(true) => {
                    // Worked a job, look for another immediately.
                }
                Ok(false) => {
                    tokio::time::sleep(poll_interval).await;
                }
                Err(error) => {
                    tracing::error!(
                        error = %error,
                        "Worker tick failed; will retry after poll interval"
                    );
                    tokio::time::sleep(poll_interval).await;
                }
            }
        }
    }

    pub async fn tick(&self) -> Result<bool, String> {
        let Some(job) = self
            .import_jobs
            .claim_next(&self.db)
            .await
            .map_err(|error| format!("failed to claim import job: {error}"))?
        else {
            return Ok(false);
        };

        self.process_claimed_job(job).await?;
        Ok(true)
    }

    async fn process_claimed_job(&self, job: ImportJob) -> Result<(), String> {
        let job_id = job.id.to_string();
        let attempt = job.attempts;

        tracing::info!(
            job_id = %job_id,
            attempt,
            status = STATUS_RUNNING,
            input_path = %job.input_path,
            "Import job started"
        );

        match self.import_products(&job).await {
            Ok(()) => {
                let updated = self
                    .import_jobs
                    .mark_succeeded(&self.db, job.id)
                    .await
                    .map_err(|error| format!("failed to mark job succeeded: {error}"))?;

                tracing::info!(
                    job_id = %job_id,
                    attempt,
                    status = %updated.status,
                    "Import job succeeded"
                );
                debug_assert_eq!(updated.status, STATUS_SUCCEEDED);
            }
            Err(failure) => {
                let updated = self
                    .import_jobs
                    .mark_failed(&self.db, job.id, failure.reason.clone())
                    .await
                    .map_err(|error| format!("failed to mark job failed: {error}"))?;

                tracing::warn!(
                    job_id = %job_id,
                    attempt,
                    status = STATUS_FAILED,
                    error_code = %failure.error_code,
                    last_error = %failure.reason,
                    "Import job failed"
                );
                debug_assert_eq!(updated.status, STATUS_FAILED);
            }
        }

        Ok(())
    }

    async fn import_products(&self, job: &ImportJob) -> Result<(), ImportFailure> {
        let bytes = fs::read(&job.input_path).await.map_err(|error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                ImportFailure {
                    error_code: "import_file_missing",
                    reason: format!("Import file was not found: {}", job.input_path),
                }
            } else {
                ImportFailure {
                    error_code: "import_file_unreadable",
                    reason: format!("Import file could not be read: {}", job.input_path),
                }
            }
        })?;

        let file = parse_import_file(&bytes).map_err(|reason| ImportFailure {
            error_code: "import_file_invalid",
            reason,
        })?;

        for product in file.products {
            self.create_one(product).await?;
        }

        Ok(())
    }

    async fn create_one(&self, product: ImportProduct) -> Result<(), ImportFailure> {
        let handle = product.handle.clone();

        self.catalog
            .create_product(&self.db, product.into())
            .await
            .map_err(|error| map_catalog_error(error, &handle))?;

        Ok(())
    }
}

struct ImportFailure {
    error_code: &'static str,
    reason: String,
}

fn map_catalog_error(error: CatalogError, handle: &str) -> ImportFailure {
    match error {
        CatalogError::DuplicateProductHandle => ImportFailure {
            error_code: "duplicate_product_handle",
            reason: format!("Duplicate product handle: {handle}"),
        },
        CatalogError::ValidationFailed => ImportFailure {
            error_code: "validation_failed",
            reason: format!("Product title is required for handle: {handle}"),
        },
        CatalogError::InvalidHandle => ImportFailure {
            error_code: "validation_failed",
            reason: format!("Product handle is invalid: {handle}"),
        },
        CatalogError::ValueTooLarge { field, max } => ImportFailure {
            error_code: "validation_failed",
            reason: format!("{field} must be at most {max} for handle: {handle}"),
        },
        other => ImportFailure {
            error_code: "import_failed",
            reason: format!("Product import failed for handle {handle}: {other}"),
        },
    }
}

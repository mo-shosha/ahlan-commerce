--! requeue_import_job : (last_error?)
update import_jobs
set
    status = 'queued',
    updated_at = :updated_at
where id = :id
  and status = 'failed'
returning id, status, input_path, attempts, last_error, created_at, updated_at;

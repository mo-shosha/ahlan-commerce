--! fail_import_job (last_error?) : (last_error?)
update import_jobs
set
    status = 'failed',
    last_error = :last_error,
    updated_at = :updated_at
where id = :id
  and status = 'running'
returning id, status, input_path, attempts, last_error, created_at, updated_at;

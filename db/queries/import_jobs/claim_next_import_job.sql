--! claim_next_import_job : (last_error?)
update import_jobs
set
    status = 'running',
    attempts = attempts + 1,
    updated_at = :updated_at
where id = (
    select id
    from import_jobs
    where status = 'queued'
      and attempts < :max_attempts
    order by created_at asc, id asc
    limit 1
)
returning id, status, input_path, attempts, last_error, created_at, updated_at;

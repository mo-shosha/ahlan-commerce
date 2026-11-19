--! create_import_job (last_error?) : (last_error?)
insert into import_jobs
    (id, status, input_path, attempts, last_error, created_at, updated_at)
values
    (:id, :status, :input_path, :attempts, :last_error, :created_at, :updated_at)
returning id, status, input_path, attempts, last_error, created_at, updated_at;

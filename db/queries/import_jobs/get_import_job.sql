--! get_import_job : (last_error?)
select id, status, input_path, attempts, last_error, created_at, updated_at
from import_jobs
where id = :id;

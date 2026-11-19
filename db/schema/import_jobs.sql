create table import_jobs (
  id uuid primary key,
  status text not null,
  input_path text not null,
  attempts integer not null,
  last_error text,
  created_at timestamptz not null,
  updated_at timestamptz not null
);

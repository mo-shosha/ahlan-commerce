begin;

alter table products add column description text;
alter table products add column published_at timestamptz;

update products
set published_at = created_at
where published and published_at is null;

commit;

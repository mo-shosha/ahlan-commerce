create table products (
  id uuid primary key,
  title text not null,
  handle text not null unique,
  description text,
  price_cents integer not null,
  inventory_quantity integer not null,
  published boolean not null,
  published_at timestamptz,
  created_at timestamptz not null,
  updated_at timestamptz not null
);

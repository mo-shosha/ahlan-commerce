-- Create "products" table
CREATE TABLE "products" (
  "id" uuid NOT NULL,
  "title" text NOT NULL,
  "handle" text NOT NULL,
  "description" text NULL,
  "price_cents" integer NOT NULL,
  "inventory_quantity" integer NOT NULL,
  "published" boolean NOT NULL,
  "published_at" timestamptz NULL,
  "created_at" timestamptz NOT NULL,
  "updated_at" timestamptz NOT NULL,
  PRIMARY KEY ("id"),
  CONSTRAINT "products_handle_key" UNIQUE ("handle")
);

-- Create "import_jobs" table
CREATE TABLE "import_jobs" (
  "id" uuid NOT NULL,
  "status" text NOT NULL,
  "input_path" text NOT NULL,
  "attempts" integer NOT NULL,
  "last_error" text NULL,
  "created_at" timestamptz NOT NULL,
  "updated_at" timestamptz NOT NULL,
  PRIMARY KEY ("id")
);

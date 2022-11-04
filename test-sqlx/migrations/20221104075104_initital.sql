CREATE TYPE "source_type" AS ENUM (
  'solidity',
  'vyper',
  'yul'
);

CREATE TYPE "bytecode_type" AS ENUM (
  'creation_input',
  'deployed_bytecode'
);

CREATE TYPE "part_type" AS ENUM (
  'main',
  'metadata'
);

CREATE TYPE "verification_type" AS ENUM (
  'flattened_contract',
  'multi_part_files',
  'standard_json',
  'metadata'
);

CREATE TABLE "sources" (
  "id" SERIAL PRIMARY KEY,
  "created_at" timestamp NOT NULL DEFAULT (now()),
  "updated_at" timestamp NOT NULL DEFAULT (now()),
  "source_type" source_type NOT NULL,
  "compiler_version" varchar NOT NULL,
  "compiler_settings" jsonb NOT NULL,
  "file_name" varchar NOT NULL,
  "contract_name" varchar NOT NULL,
  "abi" jsonb,
  "raw_creation_input" bytea NOT NULL,
  "raw_deployed_bytecode" bytea NOT NULL
);

CREATE TABLE "files" (
  "id" SERIAL PRIMARY KEY,
  "created_at" timestamp NOT NULL DEFAULT (now()),
  "updated_at" timestamp NOT NULL DEFAULT (now()),
  "name" varchar NOT NULL,
  "content" varchar NOT NULL
);

CREATE TABLE "source_files" (
  "id" SERIAL PRIMARY KEY,
  "created_at" timestamp NOT NULL DEFAULT (now()),
  "updated_at" timestamp NOT NULL DEFAULT (now()),
  "source_id" bigserial,
  "file_id" bigserial
);

CREATE TABLE "bytecodes" (
  "id" SERIAL PRIMARY KEY,
  "created_at" timestamp NOT NULL DEFAULT (now()),
  "updated_at" timestamp NOT NULL DEFAULT (now()),
  "source_id" bigserial NOT NULL,
  "type" bytecode_type NOT NULL
);

CREATE TABLE "parts" (
  "id" SERIAL PRIMARY KEY,
  "created_at" timestamp NOT NULL DEFAULT (now()),
  "updated_at" timestamp NOT NULL DEFAULT (now()),
  "type" part_type NOT NULL,
  "data" bytea NOT NULL
);

CREATE TABLE "bytecode_parts" (
  "id" SERIAL PRIMARY KEY,
  "created_at" timestamp NOT NULL DEFAULT (now()),
  "updated_at" timestamp NOT NULL DEFAULT (now()),
  "bytecode_id" bigserial NOT NULL,
  "part_id" bigserial NOT NULL,
  "order" bigint NOT NULL
);

CREATE TABLE "verified_contracts" (
  "id" SERIAL PRIMARY KEY,
  "created_at" timestamp NOT NULL DEFAULT (now()),
  "updated_at" timestamp NOT NULL DEFAULT (now()),
  "source_id" bigserial,
  "raw_bytecode" bytea NOT NULL,
  "bytecode_type" bytecode_type NOT NULL,
  "verification_settings" jsonb NOT NULL,
  "verification_type" verification_type NOT NULL
);

CREATE UNIQUE INDEX ON "source_files" ("source_id", "file_id");

CREATE UNIQUE INDEX ON "parts" ("type", "data");

CREATE UNIQUE INDEX ON "bytecode_parts" ("bytecode_id", "order");

COMMENT ON TABLE "sources" IS 'The main table that contains details of source files compilations';

COMMENT ON COLUMN "sources"."abi" IS 'May be null if source type is "Yul"';

COMMENT ON COLUMN "sources"."raw_creation_input" IS 'The result of local compilation. May be used for searhing for full matches';

COMMENT ON COLUMN "sources"."raw_deployed_bytecode" IS 'The result of local compilation. May be used for searching for full matches';

COMMENT ON TABLE "verified_contracts" IS 'The table contains historic data that are not required for the verificaiton     in general, but what we still would like to store as it may be useful for     further processing. Contains information about contracts being verified via
the service.';

ALTER TABLE "source_files" ADD FOREIGN KEY ("source_id") REFERENCES "sources" ("id");

ALTER TABLE "source_files" ADD FOREIGN KEY ("file_id") REFERENCES "files" ("id");

ALTER TABLE "bytecodes" ADD FOREIGN KEY ("source_id") REFERENCES "sources" ("id");

ALTER TABLE "bytecode_parts" ADD FOREIGN KEY ("bytecode_id") REFERENCES "bytecodes" ("id");

ALTER TABLE "bytecode_parts" ADD FOREIGN KEY ("part_id") REFERENCES "parts" ("id");

ALTER TABLE "verified_contracts" ADD FOREIGN KEY ("source_id") REFERENCES "sources" ("id");

use sea_orm_migration::{
    prelude::*,
    sea_orm::{ConnectionTrait, DbBackend, Statement, StatementBuilder},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let stmnts = vec![
            r#"
            CREATE TYPE "source_type" AS ENUM (
            'solidity',
            'vyper',
            'yul'
            );
            "#,
            r#"
            CREATE TYPE "bytecode_type" AS ENUM (
            'creation_input',
            'deployed_bytecode'
            );
            "#,
            r#"
            CREATE TYPE "part_type" AS ENUM (
            'main',
            'metadata'
            );
            "#,
            r#"
            CREATE TYPE "verification_type" AS ENUM (
            'flattened_contract',
            'multi_part_files',
            'standard_json',
            'metadata'
            );
            "#,
            r#"
            CREATE TABLE "sources" (
            "id" SERIAL PRIMARY KEY,
            "created_at" timestamp DEFAULT (now()),
            "updated_at" timestamp DEFAULT (now()),
            "source_type" source_type NOT NULL,
            "compiler_version" varchar NOT NULL,
            "compiler_settings" jsonb NOT NULL,
            "file_name" varchar NOT NULL,
            "contract_name" varchar NOT NULL,
            "abi" jsonb,
            "raw_creation_input" bytea NOT NULL,
            "raw_deployed_bytecode" bytea NOT NULL
            );
            "#,
            r#"
            CREATE TABLE "bytecodes" (
            "id" SERIAL PRIMARY KEY,
            "created_at" timestamp DEFAULT (now()),
            "updated_at" timestamp DEFAULT (now()),
            "source_id" bigserial,
            "type" bytecode_type NOT NULL
            );
            "#,
            r#"
            CREATE TABLE "parts" (
            "id" SERIAL PRIMARY KEY,
            "created_at" timestamp DEFAULT (now()),
            "updated_at" timestamp DEFAULT (now()),
            "type" part_type NOT NULL,
            "data" bytea NOT NULL
            );
            "#,
            r#"
            CREATE TABLE "bytecode_parts" (
            "id" SERIAL PRIMARY KEY,
            "created_at" timestamp DEFAULT (now()),
            "updated_at" timestamp DEFAULT (now()),
            "bytecode_id" bigserial,
            "part_id" bigserial,
            "order" bigint
            );
            "#,
            r#"
            CREATE TABLE "files" (
            "id" SERIAL PRIMARY KEY,
            "created_at" timestamp DEFAULT (now()),
            "updated_at" timestamp DEFAULT (now()),
            "name" varchar NOT NULL,
            "content" varchar NOT NULL
            );
            "#,
            r#"
            CREATE TABLE "source_files" (
            "id" SERIAL PRIMARY KEY,
            "created_at" timestamp DEFAULT (now()),
            "updated_at" timestamp DEFAULT (now()),
            "source_id" bigserial,
            "file_id" bigserial
            );
            "#,
            r#"
            CREATE TABLE "verified_contracts" (
            "id" SERIAL PRIMARY KEY,
            "created_at" timestamp DEFAULT (now()),
            "updated_at" timestamp DEFAULT (now()),
            "source_id" bigserial,
            "raw_bytecode" bytea NOT NULL,
            "bytecode_type" bytecode_type NOT NULL,
            "verification_settings" jsonb NOT NULL,
            "verification_type" verification_type NOT NULL
            );
            "#,
            r#"
            CREATE UNIQUE INDEX ON "parts" ("type", "data");
            "#,
            r#"
            CREATE UNIQUE INDEX ON "bytecode_parts" ("bytecode_id", "order");
            "#,
            r#"
            CREATE UNIQUE INDEX ON "source_files" ("source_id", "file_id");
            "#,
            r#"
            COMMENT ON TABLE "sources" IS 'The main table that contains details of source files compilations';
            "#,
            r#"
            COMMENT ON COLUMN "sources"."abi" IS 'May be null if source type is "Yul"';
            "#,
            r#"
            COMMENT ON COLUMN "sources"."raw_creation_input" IS 'The result of local compilation. May be used for searhing for full matches';
            "#,
            r#"
            COMMENT ON COLUMN "sources"."raw_deployed_bytecode" IS 'The result of local compilation. May be used for searching for full matches';
            "#,
            r#"
            COMMENT ON TABLE "verified_contracts" IS 'The table contains historic data that are not required for the verificaiton     in general, but what we still would like to store as it may be useful for     further processing. Contains information about contracts being verified via
            the service.';
            "#,
            r#"
            ALTER TABLE "bytecodes" ADD FOREIGN KEY ("source_id") REFERENCES "sources" ("id");
            "#,
            r#"
            ALTER TABLE "bytecode_parts" ADD FOREIGN KEY ("bytecode_id") REFERENCES "bytecodes" ("id");
            "#,
            r#"
            ALTER TABLE "bytecode_parts" ADD FOREIGN KEY ("part_id") REFERENCES "parts" ("id");
            "#,
            r#"
            ALTER TABLE "source_files" ADD FOREIGN KEY ("source_id") REFERENCES "sources" ("id");
            "#,
            r#"
            ALTER TABLE "source_files" ADD FOREIGN KEY ("file_id") REFERENCES "files" ("id");
            "#,
            r#"
            ALTER TABLE "verified_contracts" ADD FOREIGN KEY ("source_id") REFERENCES "sources" ("id");"#,
        ];

        for st in stmnts.into_iter() {
            manager
                .get_connection()
                .execute(Statement::from_string(
                    manager.get_database_backend(),
                    st.to_string(),
                ))
                .await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
            DROP TABLE IF EXISTS "verified_contracts", "source_files", "files", "bytecode_parts", "parts", "bytecodes", "sources";
            "#.to_string(),
        )).await?;
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                r#"
            DROP TYPE IF EXISTS "verification_type", "source_type", "bytecode_type", "part_type";
            "#
                .to_string(),
            ))
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Contract {
    Table,
    Id,
    Name,
}

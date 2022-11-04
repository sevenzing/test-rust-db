use std::{net::SocketAddr, sync::Arc};

use async_trait::async_trait;
use futures::executor::block_on;
use test_sqlx::proto::blockscout::eth_bytecode_db::v1::eth_bytecode_db_server::EthBytecodeDb;

use test_sqlx::proto::blockscout::eth_bytecode_db::v1::{Contract, ContractByBytecode};

const DATABASE_URL: &str = "postgres://postgres:admin@localhost:5432";
const DB_NAME: &str = "bytecodes_ethereum";
use sqlx::{postgres::PgPoolOptions, PgPool};

struct Service {
    db: PgPool,
}

impl Service {
    fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl EthBytecodeDb for Service {
    async fn get_contact(
        &self,
        request: tonic::Request<ContractByBytecode>,
    ) -> Result<tonic::Response<Contract>, tonic::Status> {
        let bytecode = hex::decode(request.into_inner().bytecode)
            .map_err(|e| tonic::Status::invalid_argument(e.to_string()))?;

        let (id, contract_name, abi) = sqlx::query!(
            r#"
            SELECT id, contract_name, abi
            FROM sources
            WHERE
                raw_creation_input = $1
            "#,
            bytecode
        )
        .fetch_optional(&self.db)
        .await
        .map_err(|e| tonic::Status::invalid_argument(e.to_string()))?
        .map(|e| (e.id, e.contract_name.clone(), e.abi.clone()))
        .unwrap_or_default();

        Ok(tonic::Response::new(Contract {
            id,
            name: contract_name,
            content: abi.unwrap_or_default().to_string(),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // BEFORE:
    // $ export DATABASE_URL=postgres://postgres:admin@localhost:5432/bytecodes_ethereum
    // $ sqlx database create
    // & sqlx migrate run
    let db_url = format!("{}/{}", DATABASE_URL, DB_NAME);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;
    let eth = Arc::new(Service::new(pool));
    test_sqlx::server::http_server(eth, "0.0.0.0:4445".parse().unwrap()).await?;
    Ok(())
}

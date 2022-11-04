use std::{net::SocketAddr, sync::Arc};

use async_trait::async_trait;
use futures::executor::block_on;
use sea_orm::{
    sea_query::Expr, ConnectionTrait, Database, DatabaseConnection, DbBackend, DbErr, EntityTrait,
    QueryFilter, Statement,
};
use test_orm::proto::blockscout::eth_bytecode_db::v1::eth_bytecode_db_server::EthBytecodeDb;

use test_orm::proto::blockscout::eth_bytecode_db::v1::{Contract, ContractByBytecode};

const DATABASE_URL: &str = "postgres://postgres:admin@localhost:5432";
const DB_NAME: &str = "bytecodes_ethereum";

use entity::*;

struct Service {
    db: DatabaseConnection,
}
impl Service {
    fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl EthBytecodeDb for Service {
    async fn get_contact(
        &self,
        request: tonic::Request<ContractByBytecode>,
    ) -> Result<tonic::Response<Contract>, tonic::Status> {
        let db = &self.db;
        let bytecode = hex::decode(request.into_inner().bytecode)
            .map_err(|e| tonic::Status::invalid_argument(e.to_string()))?;
        let sources_orm = sources::Entity::find()
            .filter(Expr::col(sources::Column::RawCreationInput).eq(bytecode.clone()))
            .all(db)
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?;
        let sources_raw = sources::Entity::find()
            .from_raw_sql(Statement::from_sql_and_values(
                DbBackend::Postgres,
                r#"SELECT * FROM sources WHERE raw_creation_input = $1"#,
                vec![bytecode.into()],
            ))
            .all(db)
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?;
        assert_eq!(sources_orm, sources_raw);
        let (id, name) = sources_orm
            .iter()
            .next()
            .map(|e| (e.id, e.contract_name.clone()))
            .unwrap_or_default();
        let contract = Contract {
            id,
            name,
            content: "content".to_string(),
        };

        Ok(tonic::Response::new(contract))
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Database::connect(DATABASE_URL)
    //     .await?
    //     .execute(Statement::from_string(
    //         DbBackend::Postgres,
    //         format!("CREATE DATABASE \"{}\";", DB_NAME),
    //     ))
    //     .await?;

    let db = {
        let url = format!("{}/{}", DATABASE_URL, DB_NAME);
        Database::connect(&url).await?
    };
    let eth = Arc::new(Service::new(db));
    test_orm::server::http_server(eth, "0.0.0.0:4445".parse().unwrap()).await?;
    Ok(())
}

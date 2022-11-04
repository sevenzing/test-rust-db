use crate::proto::blockscout::eth_bytecode_db::v1::{
    eth_bytecode_db_actix::route_eth_bytecode_db,
    eth_bytecode_db_server::{EthBytecodeDb, EthBytecodeDbServer},
};
use actix_web::{web::ServiceConfig, App, HttpServer};
use std::{net::SocketAddr, sync::Arc};

pub fn http_server(eth: Arc<impl EthBytecodeDb>, addr: SocketAddr) -> actix_web::dev::Server {
    let server = HttpServer::new(move || {
        App::new().configure(|config| route_eth_bytecode_db(config, eth.clone()))
    })
    .bind(addr)
    .unwrap_or_else(|_| panic!("failed to bind server"));

    server.run()
}

mod service;
mod pb;
mod settings;
mod qr;

use std::env::Args;
use tonic::{transport::Server, Request, Response, Status};

use pb::qr_gen_pb::qr_generator_server::QrGeneratorServer;

use ::clap::{Parser};
use dotenv::dotenv;


async fn main() -> Result<(), Box<dyn std::error::Error>> {

    dotenv().ok();
    let args = Args::parse();

    let addr = format!("{}:{}", args.host, args.port).parse()?;
    let echo_service = MyEchoService::default();

    Server::builder()
        .add_service(EchoServiceServer::new(echo_service))
        .serve(addr)
        .await?;

    Ok(())
}
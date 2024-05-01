use clap::{Arg, Command};
use tonic::transport::Server;

use pb::qr_gen_pb::qr_generator_server::QrGeneratorServer;

use crate::service::service::QrGen;
use crate::settings::settings::Settings;

mod service;
mod pb;
mod settings;
mod qr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let command = Command::new("QR Code Generator")
        .version("1.0")
        .about("QR Code generator microservice written in Rust")
        .arg(Arg::new("config")
            .short('c')
            .long("config")
            .help("Configuration file location")
            .default_value("config.json"));

    let matches = command.get_matches();
    let config_location = matches.get_one::<String>("config").unwrap_or(&"".to_string()).to_string();
    let settings = Settings::new(&config_location, "QR_GEN")?;

    let addr = format!("{}:{}", settings.server.host, settings.server.port).parse()?;
    let qr_gen_service = QrGen::default();

    Server::builder()
        .add_service(QrGeneratorServer::new(qr_gen_service))
        .serve(addr)
        .await?;

    Ok(())
}
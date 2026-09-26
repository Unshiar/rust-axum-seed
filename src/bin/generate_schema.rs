use axum_app::log::config::LogConfig;
use axum_app::log::init_logging;
use axum_app::schemas::*;
use clap::Parser;
use std::fs::File;
use std::io::{BufWriter, Write};
use utoipa::openapi::Server;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct SchemaArgs {
    #[arg(
        long,
        short,
        default_value = "http://localhost:8080",
        help = "The server that provides the connectivity information to target server"
    )]
    pub server: String,
}

fn main() {
    init_logging(LogConfig::default());
    let args = SchemaArgs::parse();

    tracing::info!("Generating API...");
    let mut main_api = ApiDoc::openapi();
    main_api.servers = Some(vec![Server::new(args.server)]);
    let json = main_api
        .to_pretty_json()
        .expect("Failed to serialize OpenAPI to json");

    let file = File::create("openapi.json").expect("openapi.json file creation failed");

    let mut writer = BufWriter::new(file);
    writer
        .write_all(json.as_bytes())
        .expect("log file creation failed");

    tracing::info!("done.");
}

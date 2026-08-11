use axum_app::api::*;
use axum_app::log::init_logging;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    init_logging();

    tracing::info!("Generating API...");
    let json = ApiDoc::openapi()
        .to_pretty_json()
        .expect("Failed to serialize OpenAPI to json");

    let file = File::create("openapi.json").expect("openapi.json file creation failed");

    let writer = &mut BufWriter::new(file);
    writer
        .write_all(json.as_bytes())
        .expect("log file creation failed");

    tracing::info!("done.");
}

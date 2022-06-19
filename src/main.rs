mod error;
mod ws;
mod db;
mod constants;

pub use constants::*;
use db::*;
pub use error::*;

use ws::*;
use std::env::current_dir;
use poem::listener::TcpListener;
use poem::{get, EndpointExt, Route, Server};
use poem::endpoint::StaticFilesEndpoint;
use tracing::{span, event, Level, info};
use std::sync::{RwLock, Arc};

#[tokio::main]
async fn main() -> DnResult<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let conn = connect_to_database()?;
    let shared_conn = Arc::new(RwLock::new(conn));

    let app = Route::new()
        .nest("/", get_static_endpoint()?)
        .at("/ws/:name", get(ws.data(tokio::sync::broadcast::channel::<String>(32).0)),);

    info!("Starting server");
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await?;

    Ok(())
}

fn get_static_endpoint() -> DnResult<StaticFilesEndpoint> {
    let mut current = current_dir()?;
    current.push("res\\static");

    Ok(StaticFilesEndpoint::new(current)
        .show_files_listing()
        .index_file("index.html"))
}
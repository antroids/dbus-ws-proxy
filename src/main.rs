use crate::signal_handler::SignalHandler;
use crate::web_socket_message_handler::WebSocketMessageHandler;
use axum::extract::ws::{Message, WebSocket};
use axum::extract::{Query, WebSocketUpgrade};
use axum::http::header;
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::get;
use axum::Router;
use clap::Parser;
use error::{Error, RequestError};
use log::warn;
use message::OutputMessage;
use serde::Deserialize;
use state::WebSocketState;
use std::fmt::Debug;
use std::net::SocketAddr;
use std::ops::ControlFlow;
use thiserror::Error;
use tracing::{error, info, instrument};

mod error;
mod message;
mod signal_handler;
mod state;
mod value;
mod web_socket_message_handler;

type Result<T> = std::result::Result<T, Error>;
type RequestResult<T> = std::result::Result<T, RequestError>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "127.0.0.1:2024")]
    address: SocketAddr,

    #[arg(short, long, value_enum, default_value = "Warn")]
    log_level: tracing::level_filters::LevelFilter,
}

#[tokio::main]
#[instrument]
async fn main() {
    let args = Args::parse();

    tracing_subscriber::fmt()
        .with_max_level(args.log_level)
        .init();

    info!("Server starting with arguments: {:?}", args);

    listen(args.address).await;
}

#[instrument]
async fn listen(address: SocketAddr) {
    let app = Router::new()
        .with_state(WebSocketParameters::default())
        .route("/", get(|| async { Redirect::permanent("/api") }))
        .route("/api", get(asyncapi_schema_handler))
        .route("/ws/v1", get(web_socket_handler));
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .unwrap_or_else(|e| {
            error!(
                "Unable to bind TCP listener to address '{:?}': {}",
                address, e
            );
            panic!()
        });
    let _server = axum::serve(listener, app.into_make_service())
        .await
        .inspect_err(|e| {
            error!("Unable to initialize server: {}", e);
        });
}

#[derive(Default, Debug, Deserialize, Clone)]
struct WebSocketParameters {
    #[serde(default)]
    connection: DBusConnectionTarget,
}

#[derive(Default, Debug, Deserialize, Copy, Clone)]
#[serde(rename_all_fields = "camelCase")]
enum DBusConnectionTarget {
    #[default]
    Session,
    System,
}

async fn asyncapi_schema_handler() -> impl IntoResponse {
    let asyncapi_schema = include_str!("../dbus-websocket-asyncapi.json");
    (
        [(header::CONTENT_TYPE, "application/json")],
        asyncapi_schema,
    )
}

#[instrument]
async fn web_socket_handler(
    Query(params): Query<WebSocketParameters>,
    ws: WebSocketUpgrade,
) -> Response {
    ws.on_failed_upgrade(|err| error!("WebSocket initialization failed: {}", err))
        .on_upgrade(move |ws| handle_web_socket_upgrade(params.clone(), ws))
}

pub(crate) trait WebSocketEventHandler<'a, T>
where
    T: 'a,
    T: Send + Sync,
{
    async fn handle(&self, event: T) -> ControlFlow<Option<OutputMessage>, Option<OutputMessage>>;
}

#[instrument]
async fn handle_web_socket_upgrade(params: WebSocketParameters, mut socket: WebSocket) {
    let dbus_connection = match dbus_connection(&params).await {
        Ok(connection) => connection,
        Err(err) => {
            error!("Cannot connect to the Session bus: {}", err);
            return;
        }
    };
    let state = WebSocketState::default();
    let web_socket_message_handler = WebSocketMessageHandler::new(&dbus_connection, &state);
    let signal_handler = SignalHandler::default();

    loop {
        let control = tokio::select! {
            msg = next_web_socket_message(&mut socket) => {
                web_socket_message_handler.handle(msg).await
            },
            Some(signal) = state.signals().next() => {
                signal_handler.handle(signal).await
            }
        };
        match control {
            ControlFlow::Continue(Some(msg)) => {
                let _ = send_output_message(&mut socket, &msg).await;
            }
            ControlFlow::Continue(None) => {}
            ControlFlow::Break(msg) => {
                if let Some(msg) = msg {
                    let _ = send_output_message(&mut socket, &msg).await;
                }
                break;
            }
        }
    }

    dbus_connection.graceful_shutdown().await;
}

#[instrument]
async fn next_web_socket_message(socket: &mut WebSocket) -> Result<Option<Message>> {
    match socket.recv().await {
        Some(Ok(msg)) => Ok(Some(msg)),
        Some(Err(err)) => {
            warn!(
                "WebSocket error occurred or connection was interrupted: {}",
                err
            );
            Err(err.into())
        }
        None => {
            info!("WebSocket connection closed");
            Ok(None)
        }
    }
}

async fn send_output_message(socket: &mut WebSocket, output_message: &OutputMessage) -> Result<()> {
    let json = serde_json::to_string(output_message)?;
    Ok(socket.send(Message::Text(json)).await?)
}

#[instrument]
async fn dbus_connection(params: &WebSocketParameters) -> Result<zbus::Connection> {
    let builder = match params.connection {
        DBusConnectionTarget::Session => zbus::connection::Builder::session()?,
        DBusConnectionTarget::System => zbus::connection::Builder::system()?,
    };
    Ok(builder.build().await?)
}

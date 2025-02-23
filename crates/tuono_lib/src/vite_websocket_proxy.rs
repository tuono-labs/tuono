use crate::config::GLOBAL_CONFIG;
use axum::extract::ws::{self, Utf8Bytes as AxumUtf8Bytes, WebSocket, WebSocketUpgrade};
use axum::response::IntoResponse;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::{Error, Message};
use tungstenite::client::IntoClientRequest;
use tungstenite::ClientRequestBuilder;

const VITE_WS_PROTOCOL: &str = "vite-hmr";

/// This is the entry point to proxy the vite's WebSocket.
/// The proxy is needed for allowing all the development requests to point
/// to localhost:3000/*. This enabled the framework to be developed in a Docker
/// environment with just the 3000 port exposed.
pub async fn vite_websocket_proxy(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.protocols([VITE_WS_PROTOCOL]).on_upgrade(handle_socket)
}

async fn handle_socket(mut tuono_socket: WebSocket) {
    // Send back a message to confirm connection
    if tuono_socket
        .send(ws::Message::Ping("tuono connected".into()))
        .await
        .is_err()
    {
        // If is error close the connection
        return;
    }

    let config = GLOBAL_CONFIG
        .get()
        .expect("Failed to get the internal config");

    let vite_ws = format!(
        "ws://{}:{}/vite-server/",
        config.server.host,
        config.server.port + 1
    );

    let vite_ws_request = ClientRequestBuilder::new(vite_ws.parse().unwrap())
        .with_sub_protocol(VITE_WS_PROTOCOL)
        .into_client_request()
        .expect("Failed to create vite WS request");

    let vite_socket = match connect_async(vite_ws_request).await {
        Ok((stream, _)) => {
            // Connected to vite's WebSocket
            stream
        }
        Err(e) => {
            eprintln!("Failed to connect to vite's WebSocket. Error: {e}");
            // As fallback vite automatically connect to port 3001.
            return;
        }
    };

    let (mut vite_sender, mut vite_receiver) = vite_socket.split();
    let (mut tuono_sender, mut tuono_receiver) = tuono_socket.split();

    // Handle browser messages.
    // Every message gets forwarded to the vite WebSocket
    tokio::spawn(async move {
        while let Some(msg) = tuono_receiver.next().await {
            if let Ok(msg) = msg {
                let msg_to_vite = match msg.clone() {
                    ws::Message::Text(str) => Message::Text(str.to_string().into()),
                    ws::Message::Pong(payload) => Message::Pong(payload.into()),
                    ws::Message::Ping(payload) => Message::Ping(payload.into()),
                    ws::Message::Binary(payload) => Message::Binary(payload.into()),
                    // Hard to match axum and tungstenite close payload.
                    // Not a priority
                    ws::Message::Close(_) => Message::Close(None),
                };

                vite_sender
                    .send(msg_to_vite)
                    .await
                    .expect("Failed to tunnel msg to vite's WebSocket");

                msg
            } else {
                // Close browser's WebSocket connection.
                return;
            };
        }
    });

    // Handle vite messages.
    // Every message gets forwarded to the browser.
    tokio::spawn(async move {
        while let Some(Ok(msg)) = vite_receiver.next().await {
            let msg_to_browser = match msg {
                Message::Text(str) => ws::Message::Text(AxumUtf8Bytes::from(str.to_string())),
                Message::Ping(payload) => ws::Message::Ping(payload.into()),
                Message::Pong(payload) => ws::Message::Pong(payload.into()),
                Message::Binary(payload) => ws::Message::Binary(payload.into()),
                // Hard to match axum and tungstenite close payload.
                // Not a priority
                Message::Close(_) => ws::Message::Close(None),
                _ => {
                    eprintln!("Unexpected message from the vite WebSocket to the browser: {msg:?}");
                    ws::Message::Text("Unhandled".to_string().into())
                }
            };

            if let Err(err) = tuono_sender.send(msg_to_browser).await {
                if err.to_string() != Error::AlreadyClosed.to_string() {
                    eprintln!("Failed to send back message from vite to browser: {err}")
                }
            }
        }
    });
}

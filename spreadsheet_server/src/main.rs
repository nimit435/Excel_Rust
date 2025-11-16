use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use serde_json;
use serde::{Deserialize, Serialize};
use spreadsheet_core::{parse_input, Sheet, ClientMsg, ServerMsg}; // Use your library!
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::{broadcast, Mutex};

// Define the messages we'll send over the WebSocket


struct AppState {
    // The one and only "master" spreadsheet
    sheet: Mutex<Sheet>,
    // The broadcast channel to send updates to all users
    tx: broadcast::Sender<ServerMsg>,
}

#[tokio::main]
async fn main() {
    // 1. Create the master sheet
    let sheet = Sheet::create_sheet(20, 20);

    // 2. Create the broadcast channel
    // The channel can hold 100 messages if receivers are slow
    let (tx, _rx) = broadcast::channel(100);

    // 3. Create the shared state
    // We wrap it in an `Arc` (Atomic Reference Counter)
    // to share it safely across all user connections
    let app_state = Arc::new(AppState {
        sheet: Mutex::new(sheet),
        tx,
    });

    // 4. Define the web routes
    let app = Router::new()
        .route("/ws", get(websocket_handler)) // Our WebSocket endpoint
        .with_state(app_state); // Make the state available to the handler

    // 5. Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server listening on ws://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// This function is called when a user connects to /ws
async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    // Finalize the WebSocket upgrade and pass it to our connection handler
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

// This function handles a single user's WebSocket connection
async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
    // Subscribe this user to the broadcast channel
    let mut rx = state.tx.subscribe();

    // --- Send the initial state ---
    // Get a lock on the master sheet
    let sheet = state.sheet.lock().await;
    let initial_msg = ServerMsg {
        sheet: sheet.clone(), // Clone the current sheet
    };
    // Send the current sheet to *just this user*
    if socket
        .send(Message::Text(serde_json::to_string(&initial_msg).unwrap()))
        .await
        .is_err()
    {
        // User disconnected before we could send
        return;
    }
    // Release the lock
    drop(sheet);

    // --- Two-way communication loop ---
    loop {
        tokio::select! {
            // 1. Listen for messages from the user (e.g., "A1=50")
            Some(Ok(msg)) = socket.recv() => {
                if let Message::Text(text) = msg {
                    // Try to parse the client's message
                    if let Ok(client_msg) = serde_json::from_str::<ClientMsg>(&text) {
                        
                        // --- Apply Update ---
                        // Get a lock on the master sheet
                        let mut sheet = state.sheet.lock().await;
                        
                        // Run your existing logic!
                        // This handles "s", "d", "A1=50", etc.
                        match parse_input(&client_msg.input, &mut sheet) {
                            Ok(_) => {
                                // Input was valid and sheet was updated
                            },
                            Err(e) => {
                                // The input was an error (e.g., cycle)
                                // We could send an error message back, but for now
                                // we'll just log it.
                                println!("Input error: {}", e);
                                // We still broadcast, as things like
                                // "scroll_to" might fail but still be valid.
                            }
                        }

                        // --- Broadcast Update ---
                        let server_msg = ServerMsg {
                            sheet: sheet.clone(), // Clone the *new* sheet state
                        };
                        
                        // Send the new sheet to the broadcast channel
                        // This will notify *all* connected users
                        let _ = state.tx.send(server_msg);

                        // Release the lock
                        drop(sheet);
                    }
                }
            },
            
            // 2. Listen for messages from the broadcast (updates from *other* users)
            Ok(server_msg) = rx.recv() => {
                // Send the new sheet state to this user
                if socket
                    .send(Message::Text(serde_json::to_string(&server_msg).unwrap()))
                    .await
                    .is_err()
                {
                    // User disconnected
                    break;
                }
            },
            
            // 3. Handle disconnection
            else => {
                // User disconnected
                break;
            }
        }
    }
}
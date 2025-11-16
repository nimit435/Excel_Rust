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
use spreadsheet_core::{parse_input, Sheet, ClientMsg, ServerMsg, is_valid_cell}; // Use your library!
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::{broadcast, Mutex};
use clap::Parser;
// Define the messages we'll send over the WebSocket

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(default_value = "20")]
    rows: u32,

    #[arg(default_value = "20")]
    cols: u32,
}

struct AppState {
    // The one and only "master" spreadsheet
    sheet: Mutex<Sheet>,
    // The broadcast channel to send updates to all users
    tx: broadcast::Sender<ServerMsg>,
}

#[tokio::main]
async fn main() {
    // 1. Create the master sheet
    let args = Args::parse();
    let sheet = Sheet::create_sheet(args.rows, args.cols);

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
    let initial_msg = ServerMsg::SheetUpdate(sheet.clone());
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
            // 1. Listen for messages from the user (e.g., "A1=50")
// 1. Listen for messages from the user (e.g., "A1=50")
            Some(Ok(msg)) = socket.recv() => {
                if let Message::Text(text) = msg {
                    if let Ok(client_msg) = serde_json::from_str::<ClientMsg>(&text) {

                        // --- Apply Update ---
                        let mut sheet = state.sheet.lock().await;
                        let input_lowercase = client_msg.input.to_lowercase();

                        // We create a Result to hold our error message
                        let mut op_result: Result<(), String> = Ok(());

                        // --- Logic from previous step ---
                        if input_lowercase.starts_with("scroll_to") {
                            let parts: Vec<&str> = client_msg.input.split_whitespace().collect();
                            if parts.len() == 2 {
                                match is_valid_cell(parts[1], &sheet) {
                                    Ok(_) => sheet.scroll_to(parts[1]),
                                    Err(e) => op_result = Err(e), // Capture error
                                }
                            } else {
                                op_result = Err("Invalid scroll_to format.".to_string()); // Capture error
                            }
                        } else {
                            match input_lowercase.as_str() {
                                "w" => sheet.scroll_up(),
                                "a" => sheet.scroll_left(),
                                "s" => sheet.scroll_down(),
                                "d" => sheet.scroll_right(),
                                _ => {
                                    match parse_input(&client_msg.input, &mut sheet) {
                                        Ok(_) => {}, // Good!
                                        Err(e) => op_result = Err(e), // Capture error
                                    }
                                }
                            }
                        }
                        // --- End Logic ---


                        // --- Send Response ---
                        match op_result {
                            Ok(_) => {
                                // SUCCESS: Broadcast the sheet update to everyone
                                let server_msg = ServerMsg::SheetUpdate(sheet.clone());
                                let _ = state.tx.send(server_msg);
                            }
                            Err(e) => {
                                // ERROR: Send error message *only* to this user
                                let err_msg = ServerMsg::Error(e);
                                let json = serde_json::to_string(&err_msg).unwrap();
                                if socket.send(Message::Text(json)).await.is_err() {
                                    break; // User disconnected
                                }
                            }
                        }
                        drop(sheet); // Release the lock
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
use gloo_net::websocket::{futures::WebSocket, Message};
use serde_json;
use spreadsheet_core::{ClientMsg, ServerMsg, Sheet}; // Our shared types!
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use futures::{StreamExt, SinkExt};
use gloo_net::Error as GlooError;  // For the error type
use web_sys::{HtmlInputElement, InputEvent, SubmitEvent}; // For browser events
use futures::stream::SplitSink;

// --- Define the state of our component ---
struct App {
    sheet: Option<Sheet>,
    ws: Option<SplitSink<WebSocket, Message>>,
    user_input: String,
    error_message: Option<String>, // <-- ADD THIS
}

// --- Define messages for component updates ---
enum Msg {
    Connected(WebSocket),
    FromServer(ServerMsg), // <-- RENAMED (from ServerUpdate)
    InputChanged(String),
    SubmitInput,
    ConnectionFailed(GlooError),
    ConnectionLost,
    ConnectionRestored(SplitSink<WebSocket, Message>),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // --- 1. Connect to the WebSocket ---
        let link = ctx.link().clone();
        spawn_local(async move {
            let ws = WebSocket::open("ws://127.0.0.1:3000/ws");
            match ws {
                Ok(ws) => {
                    link.send_message(Msg::Connected(ws));
                }
                Err(e) => {
                    link.send_message(Msg::ConnectionFailed(gloo_net::Error::JsError(e)));
                }
            }
        });

        Self {
            sheet: None,
            ws: None,
            user_input: String::new(),
            error_message: None, // <-- ADD THIS
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // --- 2. Connection is open, start listening ---
            Msg::Connected(ws) => {
                let (write, mut read) = ws.split();
                self.ws = Some(write); // Store the "write" half to send messages
                
                // Spawn a task to listen for incoming messages
                let link = ctx.link().clone();
                spawn_local(async move {
                    while let Some(msg) = read.next().await {
                        match msg {
                            Ok(Message::Text(data)) => {
                                // We got JSON from the server, parse it
                                if let Ok(server_msg) = serde_json::from_str::<ServerMsg>(&data) {
                                    link.send_message(Msg::FromServer(server_msg)); // <-- NEW
                                }
                            }
                            _ => {}
                        }
                    }
                    // Loop exited, connection is lost
                    link.send_message(Msg::ConnectionLost);
                });
                true // Re-render
            }
            
            // --- 3. Got a new sheet state from server ---
            Msg::FromServer(server_msg) => {
                match server_msg {
                    ServerMsg::SheetUpdate(sheet) => {
                        self.sheet = Some(sheet);
                        self.error_message = None; // Clear any previous error
                    }
                    ServerMsg::Error(e) => {
                        self.error_message = Some(e);
                    }
                }
                true // Re-render to show sheet update OR error
            }

            // --- 4. User is typing ---
            Msg::InputChanged(input) => {
                self.user_input = input;
                false // No need to re-render
            }
            
            // --- 5. User submitted a command ---
            Msg::SubmitInput => {
                self.error_message = None;
                if let Some(mut ws) = self.ws.take() { // <-- 1. TAKE the writer
                    let msg = ClientMsg {
                        input: self.user_input.clone(),
                    };
                    let json = serde_json::to_string(&msg).unwrap();

                    // 2. Clone the context link to send a message back
                    let link = ctx.link().clone();

                    // 3. Send the command
                    spawn_local(async move {
                        if ws.send(Message::Text(json)).await.is_err() {
                            // If send fails, the connection is lost
                            link.send_message(Msg::ConnectionLost);
                        } else {
                            // 4. If send succeeds, put the writer back
                            link.send_message(Msg::ConnectionRestored(ws)); 
                        }
                    });
                }
                self.user_input.clear(); // Clear the input box
                true // Re-render to show the cleared box
            }
            
            Msg::ConnectionFailed(e) => {
                log::error!("WS Error: {:?}", e);
                false
            }
            Msg::ConnectionLost => {
                log::error!("WS Connection Lost");
                self.ws = None;
                false
            }
            Msg::ConnectionRestored(ws) => {
                self.ws = Some(ws); // Put the writer back into the state
                false // No re-render needed
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{ "Real-Time Rust Spreadsheet" }</h1>
                { self.view_input(ctx) }
                { self.view_error() } // <-- ADD THIS
                { self.view_grid() }
            </div>
        }
    }
}

// Helper view functions to keep `view` clean
impl App {
    // Renders the input box and submit button
    fn view_input(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        
        let oninput = link.callback(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            Msg::InputChanged(input.value())
        });

        let onsubmit = link.callback(move |e: SubmitEvent| {
            e.prevent_default(); // Stop browser from refreshing
            Msg::SubmitInput
        });
        
        html! {
            <form onsubmit={onsubmit}>
                <input
                    type="text"
                    placeholder="Enter command (e.g., A1=50 or 's')"
                    value={self.user_input.clone()}
                    oninput={oninput}
                    style="width: 300px;"
                />
                <button type="submit">{ "Submit" }</button>
            </form>
        }
    }

    // Renders the spreadsheet grid as an HTML table
    fn view_grid(&self) -> Html {
        if let Some(sheet) = &self.sheet {
            let rowt = sheet.row_top;
            let colt = sheet.col_top;
            let numrows = sheet.rows;
            let numcols = sheet.cols;

            html! {
                <table style="border-collapse: collapse; margin-top: 10px;">
                    // --- Render Column Headers (A, B, C...) ---
                    <thead>
                        <tr>
                            <th style={cell_style(true)}></th> // Top-left corner
                            {
                                (colt..std::cmp::min(colt + 10, numcols)).map(|j| {
                                    // You'll need to make your hash::col_mapping public
                                    // or just do a simple one here for now
                                    let col_name = (b'A' + j as u8) as char;
                                    html!{ <th style={cell_style(true)}>{ col_name }</th> }
                                }).collect::<Html>()
                            }
                        </tr>
                    </thead>
                    // --- Render Rows (1, 2, 3...) ---
                    <tbody>
                    {
                        (rowt..std::cmp::min(rowt + 10, numrows)).map(|i| {
                            html! {
                                <tr>
                                    <td style={cell_style(true)}>{ i + 1 }</td> // Row number
                                    {
                                        (colt..std::cmp::min(colt + 10, numcols)).map(|j| {
                                            let cell = &sheet.matrix[(i * numcols + j) as usize];
                                            let display = if cell.is_valid {
                                                cell.val.to_string() // <-- FIX (converts the number to a string)
                                            } else {
                                                "ERR".to_string()
                                            };
                                            html! { <td style={cell_style(false)}>{ display }</td> }
                                        }).collect::<Html>()
                                    }
                                </tr>
                            }
                        }).collect::<Html>()
                    }
                    </tbody>
                </table>
            }
        } else {
            html! { <p>{ "Connecting to server..." }</p> }
        }
    }
    fn view_error(&self) -> Html {
        if let Some(error) = &self.error_message {
            html! {
                <p style="color: red; font-weight: bold; margin: 10px 0;">
                    { format!("Error: {}", error) }
                </p>
            }
        } else {
            html! {}
        }
    }
}

// Helper for cell styling
fn cell_style(is_header: bool) -> String {
    let base = "border: 1px solid #ccc; padding: 4px; min-width: 60px; text-align: center;";
    if is_header {
        format!("{} {}", base, "background-color: #f4f4f4; font-weight: bold;")
    } else {
        format!("{} {}", base, "background-color: #fff;")
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default()); // <-- ADD THIS
    yew::Renderer::<App>::new().render();
}
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
    selected_cell: Option<String>,
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
    SelectCell(String),
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
            selected_cell: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // --- 2. Connection is open, start listening ---
            Msg::SelectCell(cell_id) => {
                if self.selected_cell.as_ref() == Some(&cell_id) {
                    // This cell is already selected, so deselect it
                    self.selected_cell = None;
                } else {
                    // This is a new cell, so select it
                    self.selected_cell = Some(cell_id.clone());
                    self.user_input.clear(); 
                }
                true // Re-render to show border change
            }
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
                if let Some(mut ws) = self.ws.take() {
                    
                    // 1. Determine the final command to send
                    let final_command = if let Some(cell_id) = &self.selected_cell {
                        // If a cell is selected, prefix the input!
                        // User types "10", we send "A1=10"
                        format!("{}={}", cell_id, self.user_input)
                    } else {
                        // No cell selected, treat as raw command (e.g., "s", "w", "A1=50")
                        self.user_input.clone()
                    };

                    let msg = ClientMsg {
                        input: final_command, // Send the computed command
                    };
                    let json = serde_json::to_string(&msg).unwrap();
                    
                    let link = ctx.link().clone();
                    spawn_local(async move {
                        if ws.send(Message::Text(json)).await.is_err() {
                            link.send_message(Msg::ConnectionLost);
                        } else {
                            link.send_message(Msg::ConnectionRestored(ws));
                        }
                    });
                }
                self.user_input.clear();
                // We don't clear selected_cell so you can keep editing the same cell if you fail
                true
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
                { self.view_grid(ctx) }
            </div>
        }
    }
}

// Helper view functions to keep `view` clean
impl App {
    // Renders the input box and submit button
    fn view_input(&self, ctx: &Context<Self>) -> Html {
        // ... existing callback logic ...
        let link = ctx.link();
        let oninput = link.callback(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            Msg::InputChanged(input.value())
        });
        let onsubmit = link.callback(move |e: SubmitEvent| {
            e.prevent_default();
            Msg::SubmitInput
        });

        // Dynamic placeholder text
        let placeholder = if let Some(id) = &self.selected_cell {
            format!("Enter formula for {} (e.g., 10 or A1+B2)", id)
        } else {
            "Select a cell or type a command (e.g., 's' to scroll)".to_string()
        };
        
        html! {
            <form onsubmit={onsubmit}>
                <input
                    type="text"
                    placeholder={placeholder} // <-- Use dynamic placeholder
                    value={self.user_input.clone()}
                    oninput={oninput}
                    style="width: 300px; padding: 5px;"
                />
                <button type="submit" style="padding: 5px;">{ "Update" }</button>
            </form>
        }
    }

    // Renders the spreadsheet grid as an HTML table
    fn view_grid(&self, ctx: &Context<Self>) -> Html {
        if let Some(sheet) = &self.sheet {
            let rowt = sheet.row_top;
            let colt = sheet.col_top;
            let numrows = sheet.rows;
            let numcols = sheet.cols;

            html! {
                <table style="border-collapse: collapse; margin-top: 10px; cursor: default;">
                    <thead>
                        <tr>
                            <th style={cell_style(true, false)}></th>
                            {
                                (colt..std::cmp::min(colt + 10, numcols)).map(|j| {
                                    let col_name = (b'A' + j as u8) as char;
                                    html!{ <th style={cell_style(true, false)}>{ col_name }</th> }
                                }).collect::<Html>()
                            }
                        </tr>
                    </thead>
                    <tbody>
                    {
                        (rowt..std::cmp::min(rowt + 10, numrows)).map(|i| {
                            html! {
                                <tr>
                                    <td style={cell_style(true, false)}>{ i + 1 }</td>
                                    {
                                        (colt..std::cmp::min(colt + 10, numcols)).map(|j| {
                                            // 1. Calculate Cell ID (e.g., "A1")
                                            let col_char = (b'A' + j as u8) as char;
                                            let cell_id = format!("{}{}", col_char, i + 1);
                                            
                                            // 2. Check if this specific cell is selected
                                            let is_selected = self.selected_cell.as_ref() == Some(&cell_id);

                                            // 3. Create Click Handler
                                            let id_for_click = cell_id.clone();
                                            let onclick = ctx.link().callback(move |_| Msg::SelectCell(id_for_click.clone()));

                                            // 4. Get value
                                            let cell = &sheet.matrix[(i * numcols + j) as usize];
                                            let display = if cell.is_valid {
                                                cell.val.to_string()
                                            } else {
                                                "ERR".to_string()
                                            };

                                            // 5. Render with click handler and conditional style
                                            html! { 
                                                <td 
                                                    onclick={onclick} 
                                                    style={cell_style(false, is_selected)}
                                                >
                                                    { display }
                                                </td> 
                                            }
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
// Updated helper: takes `is_selected` boolean
fn cell_style(is_header: bool, is_selected: bool) -> String {
    let mut style = String::from("padding: 4px; min-width: 60px; text-align: center;");
    
    if is_header {
        style.push_str("background-color: #f4f4f4; font-weight: bold; border: 1px solid #ccc;");
    } else {
        style.push_str("background-color: #fff; cursor: pointer;");
        if is_selected {
            // Highlight selected cells!
            style.push_str("border: 2px solid #2196F3;"); 
        } else {
            style.push_str("border: 1px solid #ccc;");
        }
    }
    style
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default()); // <-- ADD THIS
    yew::Renderer::<App>::new().render();
}
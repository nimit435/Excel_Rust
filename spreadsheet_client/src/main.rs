use gloo_net::websocket::{futures::WebSocket, Message};
use serde_json;
use spreadsheet_core::{ClientMsg, ServerMsg, Sheet, col_mapping}; // Our shared types!
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use futures::{StreamExt, SinkExt};
use gloo_net::Error as GlooError;
use web_sys::{HtmlInputElement, InputEvent, FocusEvent, KeyboardEvent}; // --- CHANGED --- (Added FocusEvent)
use futures::stream::SplitSink;


// --- Define the state of our component ---
struct App {
    sheet: Option<Sheet>,
    ws: Option<SplitSink<WebSocket, Message>>,
    bar_input: String,      // --- RENAMED --- (Was user_input, now for top bar)
    edit_input: String,     // --- NEW --- (For in-cell editing)
    error_message: Option<String>,
    editing_cell: Option<String>, // --- RENAMED --- (Was selected_cell)
    input_ref: NodeRef,     // --- NEW --- (To auto-focus the in-cell input)
}

// --- Define messages for component updates ---
enum Msg {
    Connected(WebSocket),
    FromServer(ServerMsg),
    BarInputChanged(String), // --- RENAMED --- (Was InputChanged)
    SubmitBarInput,        // --- RENAMED --- (Was SubmitInput)
    EditInputChanged(String),// --- NEW --- (For in-cell input)
    SubmitCellEdit,        // --- NEW --- (To submit from cell)
    SelectCell(String),      // --- CHANGED --- (Now just selects for editing)
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
            bar_input: String::new(),      // --- RENAMED ---
            edit_input: String::new(),     // --- NEW ---
            error_message: None,
            editing_cell: None,            // --- RENAMED ---
            input_ref: NodeRef::default(), // --- NEW ---
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // --- 2. User clicks a cell to edit it ---
            Msg::SelectCell(cell_id) => {
                // If we are already editing a cell, submit it first
                if self.editing_cell.is_some() {
                     ctx.link().send_message(Msg::SubmitCellEdit);
                }
                
                // Set the new cell as editing
                self.editing_cell = Some(cell_id.clone());
                
                // We clear the edit_input. User wants to type a new command.
                // A better way would be to fetch the cell's *formula*,
                // but we only have its *value*. Starting blank is clearer.
                self.edit_input.clear(); 
                true // Re-render to show the input box
            }
            
            // --- 3. Connection is open, start listening ---
            Msg::Connected(ws) => {
                let (write, mut read) = ws.split();
                self.ws = Some(write);
                
                let link = ctx.link().clone();
                spawn_local(async move {
                    while let Some(msg) = read.next().await {
                        match msg {
                            Ok(Message::Text(data)) => {
                                if let Ok(server_msg) = serde_json::from_str::<ServerMsg>(&data) {
                                    link.send_message(Msg::FromServer(server_msg));
                                }
                            }
                            _ => {}
                        }
                    }
                    link.send_message(Msg::ConnectionLost);
                });
                true 
            }
            
            // --- 4. Got a new sheet state from server ---
            Msg::FromServer(server_msg) => {
                match server_msg {
                    ServerMsg::SheetUpdate(sheet) => {
                        self.sheet = Some(sheet);
                        self.error_message = None;
                    }
                    ServerMsg::Error(e) => {
                        self.error_message = Some(e);
                    }
                }
                true
            }

            // --- 5. User is typing in the *top bar* ---
            Msg::BarInputChanged(input) => {
                self.bar_input = input;
                false
            }
            
            // --- 6. User submitted from the *top bar* ---
            Msg::SubmitBarInput => {
                self.error_message = None;
                if let Some(mut ws) = self.ws.take() {
                    // --- CHANGED ---
                    // The top bar now *only* sends raw commands.
                    // The prefix logic is gone.
                    let final_command = self.bar_input.clone();

                    let msg = ClientMsg {
                        input: final_command,
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
                self.bar_input.clear();
                true
            }
            
            // --- 7. User is typing in a *cell* ---
            Msg::EditInputChanged(input) => {
                self.edit_input = input;
                false // The <input> handles its own state
            }

            // --- 8. User submitted from a *cell* (on blur/focus loss) ---
            Msg::SubmitCellEdit => {
                if let Some(cell_id) = self.editing_cell.take() {
                    if let Some(mut ws) = self.ws.take() {
                        // --- NEW LOGIC ---
                        // Prefix the in-cell input with its cell_id
                        let final_command = format!("{}={}", cell_id, self.edit_input);

                        let msg = ClientMsg {
                            input: final_command,
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
                    self.edit_input.clear();
                    true // Re-render to turn the cell back into text
                } else {
                    false // Nothing was being edited
                }
            }

            // --- Connection Management ---
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
                self.ws = Some(ws);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // --- NEW ---
        // A simple "click-off" handler.
        // If you click the background, it submits the cell.
        let on_bg_click = ctx.link().callback(|e: MouseEvent| {
            // Only fire if the click is on the background div itself,
            // not a child element (like the table or input).
            if e.target() == e.current_target() {
                Msg::SubmitCellEdit
            } else {
                // This is a hack to create a "No-Op" message
                Msg::BarInputChanged("".to_string()) 
            }
        });

        html! {
            <div onclick={on_bg_click} style="min-height: 100vh;">
                <h1>{ "Real-Time Rust Spreadsheet" }</h1>
                { self.view_input(ctx) }
                { self.view_error() }
                { self.view_grid(ctx) }
            </div>
        }
    }

    // --- NEW ---
    // Auto-focus the in-cell <input> after it's rendered
    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            if let Some(input) = self.input_ref.cast::<HtmlInputElement>() {
                input.focus().ok();
            }
        }
    }
}

// Helper view functions to keep `view` clean
impl App {
    // --- CHANGED ---
    // Renders the *top* input bar
    // Renders the *top* input bar
    fn view_input(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        
        let oninput = link.callback(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            Msg::BarInputChanged(input.value())
        });

        // --- NEW: Add onkeydown handler for the top bar ---
        let onkeydown = link.batch_callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                e.prevent_default(); // Stop enter from doing anything else
                Some(Msg::SubmitBarInput)
            } else {
                None
            }
        });

        // --- NEW: onclick for the button ---
        let onclick = link.callback(|_| Msg::SubmitBarInput);
        
        // Placeholder is now static, as this bar is only for raw commands
        let placeholder = "Type a raw command (e.g., 's' to scroll or 'A1=50')".to_string();
        
        html! {
            // --- We removed the <form> tag ---
            <div>
                <input
                    type="text"
                    {placeholder}
                    value={self.bar_input.clone()}
                    {oninput}
                    {onkeydown} // <-- ATTACH onkeydown
                    style="width: 300px; padding: 5px;"
                />
                // --- Button now uses onclick ---
                <button {onclick} style="padding: 5px;">{ "Send Command" }</button>
            </div>
        }
    }

    // --- CHANGED ---
    // Renders the spreadsheet grid
    // Now renders an <input> for the cell being edited
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
                                    let col_name = col_mapping(j+1);
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
                                            let col_char = (b'A' + j as u8) as char;
                                            let cell_id = format!("{}{}", col_char, i + 1);
                                            
                                            // --- CHANGED ---
                                            // Check if this cell is being edited
                                            let is_editing = self.editing_cell.as_ref() == Some(&cell_id);
                                            
                                            let cell = &sheet.matrix[(i * numcols + j) as usize];
                                            let display = if cell.is_valid {
                                                cell.val.to_string()
                                            } else {
                                                "ERR".to_string()
                                            };

                                            if is_editing {
                                                // This cell is being edited: render an <input>
                                                let oninput = ctx.link().callback(|e: InputEvent| {
                                                    let input: HtmlInputElement = e.target_unchecked_into();
                                                    Msg::EditInputChanged(input.value())
                                                });
                                                let onblur = ctx.link().callback(|_: FocusEvent| {
                                                    Msg::SubmitCellEdit
                                                });

                                                // --- NEW: Add onkeydown handler ---
                                                let onkeydown = ctx.link().batch_callback(|e: KeyboardEvent| {
                                                    if e.key() == "Enter" {
                                                        e.prevent_default(); // Stop enter from adding a newline
                                                        Some(Msg::SubmitCellEdit)
                                                    } else {
                                                        None // Do nothing on other keys
                                                    }
                                                });

                                                html! {
                                                    <td style={cell_style(false, true)}>
                                                        <input
                                                            type="text"
                                                            value={self.edit_input.clone()}
                                                            {oninput}
                                                            {onblur}
                                                            {onkeydown} // <-- ATTACH HANDLER
                                                            ref={self.input_ref.clone()} // For auto-focus
                                                            style="width: 100%; border: none; padding: 0; margin: 0; text-align: center;"
                                                        />
                                                    </td>
                                                }
                                            }
                                            else {
                                                // --- CHANGED ---
                                                // This cell is static text: render text
                                                let id_for_click = cell_id.clone();
                                                let onclick = ctx.link().callback(move |_| Msg::SelectCell(id_for_click.clone()));
                                                html! { 
                                                    <td {onclick} style={cell_style(false, false)}>
                                                        { display }
                                                    </td> 
                                                }
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
    
    // Renders the error message (no change)
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
// `is_selected` now means "is being edited"
fn cell_style(is_header: bool, is_selected: bool) -> String {
    let mut style = String::from("padding: 4px; min-width: 60px; text-align: center;");
    
    if is_header {
        style.push_str("background-color: #f4f4f4; font-weight: bold; border: 1px solid #ccc;");
    } else {
        style.push_str("background-color: #fff;");
        if is_selected {
            // --- CHANGED ---
            // Style for the <td> *containing* the input
            style.push_str("border: 2px solid #2196F3; padding: 0;"); 
        } else {
            // Style for a normal, clickable cell
            style.push_str("border: 1px solid #ccc; cursor: pointer;");
        }
    }
    style
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
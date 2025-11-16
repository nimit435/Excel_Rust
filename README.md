# Real-Time Rust Spreadsheet

A high-performance, real-time collaborative spreadsheet application built entirely in Rust. This project uses an Axum backend with WebSockets to synchronize a live spreadsheet state with a reactive Yew frontend.

The core spreadsheet engine is designed for efficiency and can handle very large sheets and complex formula parsing.

## 🚀 Features

* **High-Performance Engine:** The core logic is built for speed and can efficiently manage and compute **very large sheets** with minimal overhead.
* **Complex Formula Parsing:** Supports standard spreadsheet functions like **`SUM(A1:ZZ99)`** as well as cell references and basic arithmetic (e.g., `A1+B2`).
* **Real-Time Updates:** Changes made by one user are instantly broadcast to all other connected clients.
* **Interactive UI:** Click any cell to select it and type a value or formula directly into it.
* **Command Bar:** A top bar is also available for entering raw commands (e.g., `s` to scroll down, or `C1=50`).
* **Robust Error Handling:** Invalid formulas (e.g., `1/0` or `SUM(A1)`) return an error message to the user without crashing the sheet.

## 🛠️ Tech Stack

* **Backend:** [Axum](https://github.com/tokio-rs/axum)
* **Frontend:** [Yew](https://yew.rs/)
* **Real-Time:** WebSockets (via Axum & `gloo-net`)
* **Core Logic:** A shared Rust crate (`spreadsheet_core`) used by both client and server.
* **Build Tool:** [Trunk](https://trunkrs.dev/) for Yew/WASM.

## 📁 Project Structure

This project is a Rust workspace composed of three separate crates:

* **`spreadsheet_core`**: The powerhouse of the application. It contains all data structures (`Sheet`, `Cell`) and the core formula parsing and computation logic. It's compiled by both the server and the client.
* **`spreadsheet_server`**: The Axum backend. It serves the static frontend files and manages all WebSocket connections, handling user input and broadcasting state changes.
* **`spreadsheet_client`**: The Yew frontend application that compiles to WebAssembly. It provides the user interface and communicates with the server via WebSockets.

## 🖥️ Running Locally

This project can be run on any local machine. You will need two terminals.

### Prerequisites

* [Rust & Cargo](https://www.rust-lang.org/tools/install)
* [Trunk](https://trunkrs.dev/#install): The build tool for the Yew frontend.

    ```bash
    cargo install trunk
    ```

### Step 1: Run the Backend Server

In your first terminal, navigate to the project's **root directory** and run the server:

```bash
# From the root `excel_rust` directory
cargo run --package spreadsheet_server
```

The server will start and listen on <ws://127.0.0.1:3000>

### Step 2: Run the Frontend Client

In your second terminal, navigate to the **spreadsheet_client** directory and use Trunk to build and serve the app:

```bash
# Navigate into the client folder
cd spreadsheet_client

# Serve the app
trunk serve --open
```

Trunk will compile the Yew app to WASM, start a local web server, and automatically open <http://127.0.0.1:8080> in your browser. The app will immediately connect to your backend server.

You can open <http://127.0.0.1:8080> in multiple browser tabs to simulate different users and see the real-time collaboration in action.

### 🔮 Future Work

* Containerize the application with Docker for easy deployment.

* Deploy to a cloud service like Shuttle, Fly.io, or AWS.

* Add more complex functions (e.g. IF, VLOOKUP).

* Implement user accounts and persisted spreadsheet storage.

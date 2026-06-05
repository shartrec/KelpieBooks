# KelpieBooks

KelpieBooks is a modern, open-source accounting application designed for small to medium-sized enterprises (SMEs) and individuals. It is built entirely in Rust, leveraging a robust backend with Rocket and SQLx, and a reactive frontend with Yew and WebAssembly.

## Tech Stack

- **Backend**: Rust, [Rocket](https://rocket.rs/), [SQLx](https://github.com/launchbadge/sqlx)
- **Frontend**: Rust, [Yew](https://yew.rs/), [WebAssembly](https://webassembly.org/)
- **Database**: [PostgreSQL](https://www.postgresql.org/)

## Why

There is an ongoing need for good modern accounting systems in the *open source* eco-system. The mainstays of open source accounting, such as GnuCash, are starting to look old and tired, even though they still work well, although were very much single user systems.

I feel it is time for something new built on today's technologies. Something that will fresh and modern and be designed from the ground up as **multi-user**.  

## Getting Started: A Guide for New Developers

This guide will walk you through setting up your local development environment to build, run, and contribute to KelpieBooks.

### 1. Prerequisites

Before you begin, ensure you have the following tools installed:

- **Rust & Cargo**: [Install Rust](https://www.rust-lang.org/tools/install)
- **`cargo-watch`**: For live-reloading the backend server.
  ```sh
  cargo install cargo-watch
  ```
- **`trunk`**: For building the WebAssembly frontend.
  ```sh
  cargo install trunk
  ```
- **PostgreSQL**: A running PostgreSQL database server. You can install it locally or use a Docker container.

### 2. Database Setup

You need to create a dedicated database and user for the application.

1.  **Connect to PostgreSQL**:
    ```sh
    psql -U postgres
    ```

2.  **Create the User and Database**:
    ```sql
    -- Choose a secure password for your user
    CREATE USER kelpie_user WITH PASSWORD 'your_secure_password';

    -- Create the database and set the owner
    CREATE DATABASE kelpie_db OWNER kelpie_user;
    ```

### 3. Backend Setup

The backend requires a `.env` file for configuration, primarily for the database connection.

1.  **Navigate to the `backend` directory**:
    ```sh
    cd backend
    ```

2.  **Create a `.env` file**:
    Create a file named `.env` in the `backend` directory and add your database connection string.

    ```env
    # .env
    DATABASE_URL=postgres://kelpie_user:your_secure_password@localhost/kelpie_db
    ```

3.  **Run Database Migrations**:
    With the `.env` file in place, `sqlx-cli` can now connect to the database to run the migrations.
    ```sh
    # Ensure you are in the 'backend' directory
    sqlx database create # Should confirm it already exists
    sqlx migrate run
    ```

### 4. Frontend Setup

The frontend assets need to be built before they can be served by the backend.

1.  **Navigate to the `frontend` directory**:
    ```sh
    cd ../frontend
    ```

2.  **Build the WebAssembly Package using Trunk**:
    This command compiles the Yew application into a WebAssembly package and places the output in the `frontend/pkg` directory.
    ```sh
    trunk build -M
    ```

### 5. Running the Application

For a smooth development workflow, you should run the backend and frontend build processes concurrently in separate terminal sessions.

-   **Terminal 1: Run the Backend Server**
    Navigate to the `backend` directory and use `cargo-watch` to automatically rebuild and restart the server on any file changes.
    ```sh
    cd backend
    cargo watch -x run
    ```
    The backend server will start on `http://localhost:8000`.

-   **Terminal 2: Watch for Frontend Changes**
    Navigate to the `frontend` directory and run `trunk` in watch mode. This will automatically rebuild the WebAssembly package whenever you save a file.
    ```sh
    cd frontend
    trunk watch
    ```

-   **Access the Application**:
    Open your web browser and navigate to `http://localhost:8000`. The Rocket backend serves the frontend assets, so you can see your changes live as both `cargo-watch` and `wasm-pack` do their work.

## Contributing

We welcome contributions! Please feel free to submit a pull request or open an issue for any bugs or feature requests. Adhering to the existing code style and architectural patterns is greatly appreciated.

# Backend - Actix Web with MongoDB

This is the backend for the web application built using Actix Web (Rust) and MongoDB. It handles user registration, login, and serves as the API endpoint for the frontend.

## Table of Contents
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Configuration](#configuration)
- [Running the Server](#running-the-server)
- [API Endpoints](#api-endpoints)
- [Folder Structure](#folder-structure)
- [Contributing](#contributing)
- [License](#license)

## Prerequisites
Before running this backend, make sure you have the following installed:

- [Rust](https://www.rust-lang.org/)
- [MongoDB](https://www.mongodb.com/try/download/community) (local or cloud instance)
- [Actix Web](https://actix.rs/)

## Installation
1. Clone the repository:
    ```bash
    git clone https://github.com/yourusername/yourproject.git
    cd yourproject/backend
    ```

2. Install dependencies:
    ```bash
    cargo build
    ```

3. Set up a MongoDB instance:
   - Install and run MongoDB locally or use a cloud service like MongoDB Atlas.
   - Ensure MongoDB is running on `mongodb://localhost:27017` or update the connection string accordingly in your configuration.

## Configuration
- The default MongoDB connection string is set to `mongodb://localhost:27017`. If you're using a remote MongoDB instance, modify the MongoDB URI in the `main.rs` file:

```rust
let client = Client::with_uri_str("your_mongodb_uri").await.unwrap();

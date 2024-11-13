
# Backend Rust Actix API with MongoDB and JWT Authentication

This Rust project is a backend API server using Actix-Web for handling HTTP requests, MongoDB as the database, bcrypt for password hashing, and JSON Web Tokens (JWT) for secure authentication. This API includes endpoints for user registration, login, and accessing authenticated user data.

## Table of Contents
- [Project Overview](#project-overview)
- [Endpoints](#endpoints)
- [Function Descriptions](#function-descriptions)
- [Dependencies](#dependencies)
- [Setup and Run Instructions](#setup-and-run-instructions)

## Project Overview

The application allows users to:
1. **Register** with a unique phone number and bank account.
2. **Login** and receive a JWT token.
3. Access protected **home** endpoint, which fetches the user's account details using the token.

### Endpoints

| Method | Endpoint      | Description                 |
| ------ | ------------- | --------------------------- |
| POST   | `/register`   | Register a new user         |
| POST   | `/login`      | Login and receive JWT token |
| GET    | `/home`       | Fetch authenticated user info |

## Function Descriptions

### 1. `register`
- **Endpoint**: `/register` (POST)
- **Function**: `async fn register(data: web::Json<RegisterRequest>, client: web::Data<Client>) -> impl Responder`
- **Description**: Registers a new user with the system. It:
  - Checks if the phone number or bank account already exists in the database.
  - Hashes the password using bcrypt before storing it.
  - Inserts the user data into the MongoDB database.
- **Response**: Returns a JSON message confirming successful registration or an error if the user already exists.

### 2. `login`
- **Endpoint**: `/login` (POST)
- **Function**: `async fn login(data: web::Json<LoginRequest>, client: web::Data<Client>) -> impl Responder`
- **Description**: Logs in an existing user by verifying credentials and generating a JWT token.
  - Checks the user's phone number and verifies the password using bcrypt.
  - If the password is correct, a JWT token is created with an expiration time.
- **Response**: Returns the JWT token or an unauthorized message on failed login.

### 3. `home`
- **Endpoint**: `/home` (GET)
- **Function**: `async fn home(req: HttpRequest, client: web::Data<Client>) -> impl Responder`
- **Description**: Serves as a protected endpoint accessible only with a valid JWT token.
  - Decodes and verifies the token from the "Authorization" header.
  - Fetches and returns the user's `name`, `phone`, `bank_account`, and `balance` from the MongoDB database.
- **Response**: Returns JSON with user details or an unauthorized message if the token is invalid.

### 4. `main`
- **Function**: `#[actix_web::main] async fn main() -> std::io::Result<()>`
- **Description**: Initializes the HTTP server and sets up the application with Actix-Web.
  - Configures CORS to allow requests from any origin.
  - Sets up routes for `/register`, `/login`, and `/home`.
  - Binds the server to `127.0.0.1:8080`.

## Dependencies

The following dependencies are used in the project:

- **[actix-web](https://docs.rs/actix-web/)**: Web framework for building HTTP servers.
- **[mongodb](https://docs.rs/mongodb/)**: MongoDB driver for Rust, used for database interactions.
- **[bson](https://docs.rs/mongodb/latest/mongodb/bson/)**: BSON serialization for MongoDB documents.
- **[bcrypt](https://docs.rs/bcrypt/)**: Library for hashing and verifying passwords using bcrypt.
- **[jsonwebtoken](https://docs.rs/jsonwebtoken/)**: Library for encoding and decoding JWTs.
- **[serde](https://docs.rs/serde/)** and **[serde_json](https://docs.rs/serde_json/)**: Used for serializing and deserializing data to JSON.
- **[chrono](https://docs.rs/chrono/)**: Date and time library, used for setting token expiration.
- **[actix-cors](https://docs.rs/actix-cors/)**: Middleware for handling Cross-Origin Resource Sharing (CORS) requests.

## Setup and Run Instructions

### Prerequisites
- Rust (Nightly version recommended)
- MongoDB server (Local or Cloud instance)

### Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/Gigabyets/Backend_Rust_actix.git
   cd Backend_Rust_actix
   ```

2. Install dependencies:
   ```bash
   cargo build
   ```

3. Start MongoDB server (if it's not running).

### Running the Server
Run the server locally:
```bash
cargo run
```

The server will be available at `http://127.0.0.1:8080`.

---


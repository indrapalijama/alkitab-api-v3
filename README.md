# Alkitab API Rust

A high-performance REST API for accessing Bible content in Indonesian and English. Built with Rust and Actix Web, this API provides endpoints to retrieve Bible chapters and book metadata with support for multiple translations.

## Features

- **Bible Content Access**: Retrieve Bible chapters and book metadata
- **Multi-language Support**: Access content in Indonesian (default) and English
- **Multiple Translations**: Support for various Bible translations (TB, AYT, KJV, NIV, etc.)
- **Authentication**: Secure API access with API key authentication
- **Performance Optimized**: Built with Rust for high performance and low resource usage
- **CORS Support**: Configurable CORS settings for web applications
- **Error Handling**: Comprehensive error handling with meaningful error messages
- **Load Testing**: Includes k6 load testing scripts for performance testing

## API Endpoints

### Bible Content

- `GET /bible/read/{book}/{chapter}` - Get a specific chapter from a Bible book
  - Query parameters:
    - `version` (optional): Bible translation version (default: "tb")
  - Example: `/bible/read/Matius/1?version=tb`

- `GET /bible/find/{book}` - Get metadata for a Bible book
  - Example: `/bible/find/Matius`

## Authentication

All API endpoints require authentication using an API key. Include the API key in the request header:

```
accesskey: your_api_key_here
```

## Installation

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/indrapalijama/alkitab-api-v3.git
   cd alkitab-api-v3
   ```

2. Create a `.env` file in the root directory with the following content:
   ```
   PORT=8080
   SECRET="your_api_key_here"
   APP_AUTH_SECRET=your_secret_key_here
   RUST_ENV=development
   RUST_LOG=info
   ```

3. Build and run the application:
   ```bash
   cargo run
   ```

## Configuration

The application can be configured using:

1. Environment variables
2. Configuration files in the `config` directory
3. `.env` file

### Configuration Files

- `config/default.toml` - Default configuration
- `config/development.toml` - Development environment configuration
- `config/local.toml` - Local configuration (not committed to version control)

## Development

### Project Structure

```
alkitab-api-rust/
├── config/             # Configuration files
├── src/                # Source code
│   ├── controllers/    # API controllers
│   ├── middleware/     # Middleware (auth, etc.)
│   ├── models/         # Data models
│   ├── routes/         # API routes
│   ├── services/       # Business logic
│   ├── error.rs        # Error handling
│   ├── config.rs       # Configuration
│   ├── lib.rs          # Library exports
│   └── main.rs         # Application entry point
├── tests/              # Unit tests
├── .env                # Environment variables
├── Cargo.toml          # Rust dependencies
└── README.md           # This file
```

### Running Tests

```bash
cargo test
```

### Load Testing

The project includes k6 load testing scripts:

- `read_test.js` - Tests the `/bible/read/{book}/{chapter}` endpoint
- `find-test.js` - Tests the `/bible/find/{book}` endpoint

To run the load tests:

```bash
k6 run read_test.js
```

## To-Do

- [ ] **Verse Range Support**: Add ability to fetch specific verse ranges (e.g., `/bible/read/John/3/16-21`)
- [ ] **Cross-References**: Implement endpoint to fetch cross-references for specific verses
- [ ] **Search Functionality**: Add text search capabilities across all translations
- [ ] **Parallel View**: Support fetching multiple translations side by side
- [ ] **Commentary Integration**: Add support for Bible commentaries
- [ ] **API Rate Limiting**: Implement rate limiting for better resource management
- [ ] **Response Caching**: Add caching layer for frequently accessed content
- [ ] **Verse of the Day**: Add endpoint to fetch random/daily verses
- [ ] **Webhook Support**: Allow users to subscribe to daily verse notifications
- [x]  **OpenAPI Documentation**: Generate comprehensive API documentation using OpenAPI/Swagger

## License

[MIT License](LICENSE)

## Acknowledgments

- Bible content provided by [alkitab.mobi](https://alkitab.mobi)
- Built with [Actix Web](https://actix.rs/) 

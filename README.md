# Chronicle

A distributed commit log service implemented in Rust, following the concepts from Travis Jeffery's "Distributed Services with Go".

## About This Project

This project implements a distributed event-streaming system using Rust instead of Go, following the learning path outlined in the book "Distributed Services with Go: Your Guide to Reliable, Scalable, and Maintainable Systems" by Travis Jeffery (Pragmatic Bookshelf, 2021).

### About the Book

"Distributed Services with Go" teaches you to build distributed systems from the ground up by creating a complete, self-contained, persistent event-stream service. The book covers:

- Building networked, secure clients and servers with gRPC
- Creating observable services with metrics, logs, and traces
- Implementing service discovery and consensus algorithms
- Operating Certificate Authorities for internal TLS authentication
- Using the Raft consensus algorithm for distributed coordination
- Deploying to the cloud with Kubernetes

### This Rust Implementation

While the original book uses Go, this project translates those concepts to Rust, providing:

- A commit log data structure for storing and retrieving records
- HTTP API endpoints for producing and consuming log entries
- Thread-safe operations using Rust's ownership model
- JSON serialization for network communication

## Getting Started

### Prerequisites

- Rust 1.70+ (current MSRV)
- Cargo (comes with Rust)

### Dependencies

- **warp 0.4** - Web server framework
- **serde 1.0** - Serialization/deserialization 
- **tokio 1.48** - Async runtime

### API Endpoints

- `POST /` - Append a record to the log
- `GET /?offset=N` - Retrieve a record at the given offset

## Project Structure

```
src/
├── main.rs          # Application entry point
├── server/
│   ├── mod.rs       # Server module exports
│   ├── log.rs       # Core log data structure
│   └── http.rs      # HTTP handlers and routes
```

## Learning Goals

This project serves as a hands-on way to learn:

- Distributed systems concepts through practical implementation
- Rust's approach to concurrent, safe systems programming
- Translation patterns between Go and Rust architectures
- Building HTTP APIs with modern Rust web frameworks

## Development Status

- [x] Basic log data structure (Record, Log)
- [x] Append and read operations
- [x] HTTP server dependencies
- [ ] HTTP API endpoints
- [ ] JSON request/response handling
- [ ] Error handling and validation
- [ ] Persistence layer
- [ ] Distributed features (future)

## License

This project is for educational purposes, following along with the "Distributed Services with Go" book.

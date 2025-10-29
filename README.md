# Chronicle

A self-contained, persistent event-stream service that records and serves events in chronological order. Built using Rust instead of Go, it follows the learning path outlined in "Distributed Services with Go: Your Guide to Reliable, Scalable, and Maintainable Systems" by Travis Jeffery (Pragmatic Bookshelf, 2021).

## Project Roadmap

This project progresses through four distinct phases, each building upon the previous:

### Phase 1: Getting Started - Persisting Events in a Log
- [x] Core event log data structure
- [x] Event append and read operations
- [ ] HTTP API for event production and consumption
- [ ] Local file persistence

### Phase 2: Network - Building a Single Instance Networked Service  
- [ ] gRPC client and server implementation
- [ ] Protocol buffer definitions
- [ ] Network communication and serialization

### Phase 3: Distribute - Distributing as a Cluster
- [ ] Service discovery mechanisms
- [ ] Raft consensus algorithm implementation
- [ ] Cluster coordination and replication

### Phase 4: Deploy - Deploying the Cluster
- [ ] Kubernetes deployment configurations
- [ ] Container orchestration
- [ ] Production monitoring and observability


## Getting Started

### Prerequisites

- Rust 1.70+ (current MSRV)
- Cargo (comes with Rust)

### Dependencies

- **warp 0.4** - Web server framework
- **serde 1.0** - Serialization/deserialization 
- **tokio 1.48** - Async runtime

### API Endpoints

- `POST /` - Append an event to the log
- `GET /?offset=N` - Retrieve an event at the given offset

## Project Structure

```
src/
├── main.rs          # Application entry point
├── server/
│   ├── mod.rs       # Server module exports
│   ├── log.rs       # Core event log data structure
│   └── http.rs      # HTTP handlers and routes
```

## Learning Goals

This project serves as a hands-on way to learn:

- Event streaming and persistent log concepts
- Distributed systems patterns through practical implementation
- Rust's approach to concurrent, safe systems programming
- Translation patterns between Go and Rust architectures
- Building networked services from HTTP to gRPC to clusters

## License

This project is for educational purposes, following along with the "Distributed Services with Go" book.

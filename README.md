# Async Query Processing Pipeline

## Overview

Async query processing pipeline in Rust demonstrating EMBP patterns, trait-based design, and production-ready error handling

This project is meant to be a comprehensive example showcasing:

- **Core Solution** - Async query processing with caching and deduplication
- **Production Architecture** - Explicit Module Boundary Pattern (EMBP) for maintainable code organization
- **Professional Standards** - Comprehensive testing, documentation, and reusable design

Structured for long-term maintainability and team collaboration.

- **API abstraction layer** - Trait-based design for external API calls
- **Stream ingestion** - Consumes query streams and forwards to processor
- **Async processor** - Processes queries with caching and deduplication
- **Integration testing** - End-to-end pipeline validation

## Architecture

The project follows the **Explicit Module Boundary Pattern (EMBP)**, using `mod.rs` files as explicit gateways that control module visibility and define public APIs. 

📖 **For complete EMBP documentation, see:** [Explicit Module Boundary Pattern (EMBP)](https://github.com/JohnBasrai/architecture-patterns/blob/main/rust/embp.md)

💼 **LinkedIn discussion on EMBP:** [Why Rust Needs Better Module Patterns](https://www.linkedin.com/feed/update/urn:li:activity:7334039101353992192)

This pattern provides:

- **Explicit Dependencies** - All inter-module dependencies visible in gateway files
- **Controlled Boundaries** - Clear separation between public API and internals  
- **Refactoring Safety** - Internal changes don't break external consumers
- **Documentation** - Gateway files serve as module documentation

## Project Structure

```
query-pipeline-rs/
├── Cargo.toml
├── src/
│   ├── lib.rs              ← Main library gateway
│   ├── external.rs         ← External API simulation
│   ├── api/
│   │   ├── mod.rs          ← API module gateway
│   │   └── client.rs       ← Real API implementation
│   ├── ingestion/
│   │   ├── mod.rs          ← Ingestion module gateway
│   │   └── stream_ingest.rs ← Stream consumption logic
│   └── processor/
│       ├── mod.rs          ← Processor module gateway
│       └── processor.rs    ← Async processing with caching
└── tests/
    └── integration.rs      ← End-to-end integration tests
```

## Key Components

The enhanced implementation includes:

### API Layer (`src/api/`)
- **`ApiCaller` trait** - Abstraction for async API calls
- **`RealApi` struct** - Production implementation using external API
- **Mock support** - Testable design with dependency injection

### Ingestion (`src/ingestion/`)
- **Stream consumption** - Processes `futures::Stream` of queries
- **Channel forwarding** - Routes queries to processor via `tokio::mpsc`

### Processor (`src/processor/`)
- **Async processing** - Concurrent query handling
- **Caching layer** - Deduplicates repeated queries
- **Generic design** - Works with any `ApiCaller` implementation

## Problem Statement

- Accept a stream of query strings
- Make async API calls for each unique query
- Cache results to avoid duplicate API calls
- Process queries concurrently
- Demonstrate testing approach

**Core Challenge:** Build a production-ready async processing pipeline in Rust.

## Building and Testing

### Prerequisites
- Rust 1.70+ (2021 edition)
- Tokio async runtime

### Build
```bash
# Check compilation
cargo check

# Build the project
cargo build

# Build optimized release
cargo build --release
```

### Testing
```bash
cargo test
```

### Test Coverage

- **Unit Tests** - Mock-based testing of processor caching logic
- **Integration Tests** - End-to-end pipeline validation with real components

## Dependencies

```toml
[dependencies]
tokio = { version = "1.37", features = ["macros", "sync", "rt", "time"] }
futures = "0.3"
anyhow = "1.0"
async-trait = "0.1"
```

## Usage Example

```rust
use query_pipeline::{spawn_processor, ingest_stream};
use query_pipeline::api::RealApi;
use futures::stream;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create API implementation
    let api = Arc::new(RealApi);
    
    // Spawn processor
    let processor_tx = spawn_processor(api);
    
    // Create query stream
    let queries = vec!["query1", "query2", "query1"]; // Note: duplicate
    let query_stream = stream::iter(queries.into_iter().map(String::from));
    
    // Process queries
    ingest_stream(query_stream, processor_tx).await?;
    
    Ok(())
}
```

## Performance Features

- **Async Processing** - Non-blocking query handling
- **Caching Layer** - Eliminates duplicate API calls
- **Generic Design** - Zero-cost abstractions with static dispatch
- **Concurrent Pipeline** - Stream ingestion and processing run concurrently

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

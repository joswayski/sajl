# SAJL - Simple Async JSON Logger


![me.png](./me.png)


### ⚠️ WIP ⚠️

Async JSON logger with batched writes and colorized output for any `Serialize` type.

**Requirements:** Tokio runtime, Serde

## Usage

```bash
cargo add sajl serde tokio
```

```rust
use sajl::Logger;
use serde::Serialize;

#[derive(Serialize)]
enum Status { 
    Active, 
    Inactive,
    RateLimited { retry_after: u32 }
}

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    verified: bool,
}

#[derive(Serialize)]
struct Request {
    method: String,
    path: String,
    status: u16,
    cached: bool,
    users: Vec<User>,
    state: Status,
}

#[tokio::main]
async fn main() {
    let logger = Logger::new(None);

    logger.info(&Request {
        method: "GET".into(),
        path: "/api/users".into(),
        status: 200,
        cached: false,
        users: vec![
            User { id: 1, name: "Alice".into(), verified: true },
            User { id: 2, name: "Bob".into(), verified: false },
        ],
        state: Status::Active,
    });
    
    logger.warn(&Status::RateLimited { retry_after: 60 });
    logger.error(&"Connection timeout");
}
```

**Output:**
```json
{"level":"INFO","timestamp":"2024-10-23T15:30:45.123Z","data":{"method":"GET","path":"/api/users","status":200,"cached":false,"users":[{"id":1,"name":"Alice","verified":true},{"id":2,"name":"Bob","verified":false}],"state":"Active"}}
{"level":"WARN","timestamp":"2024-10-23T15:30:45.456Z","data":{"RateLimited":{"retry_after":60}}}
{"level":"ERROR","timestamp":"2024-10-23T15:30:45.789Z","data":"Connection timeout"}
```

## TODO

- Filtering by log level
- Structured context fields
- Global logger instance
- Moar colors
- Tests

use sajl::Logger;
use serde::Serialize;

#[derive(Serialize)]
enum Status {
    Active,
    Inactive,
    RateLimited { retry_after: u32 },
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
            User {
                id: 1,
                name: "Alice".into(),
                verified: true,
            },
            User {
                id: 2,
                name: "Bob".into(),
                verified: false,
            },
        ],
        state: Status::Active,
    });

    logger.warn(&Status::RateLimited { retry_after: 60 });
    logger.error(&"Connection timeout");
}

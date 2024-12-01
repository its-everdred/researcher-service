mod controllers;
mod router;
mod routes;
mod services;

use tracing_subscriber;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build router
    let app = router::create_router();

    // Try ports starting at 3000
    for port in 3000..3050 {
        let addr = format!("0.0.0.0:{}", port);
        match tokio::net::TcpListener::bind(&addr).await {
            Ok(listener) => {
                println!("Listening on {}", addr);
                axum::serve(listener, app).await.unwrap();
                return;
            }
            Err(_) => println!("Port {} is in use.", port),
        }
    }

    panic!("No available ports found");
}


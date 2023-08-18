//! health_check.rs

use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let endpoint = &format!("{}/health_check", &address);
    let response = client.get(endpoint).send().await.expect("Failed to execute request");

    // Observe
    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

// Spawns an instance of the app and returns the address:port as a string.
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);


    format!("http://127.0.0.1:{}", port)
}
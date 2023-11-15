//! tests/health_check.rs

// `tokio::test` is the testing equivalent of `#[tokio::main]`
// It also spares you from having to specify the `#[test]` attribute

use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app().await;
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = email_newsletter::run(listener)
        .await
        .expect("Failed to bind address");
    // Launch the server as a background task
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

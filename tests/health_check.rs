#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    // We need to bring in `reqwest`
    // to perform HTTP requests agains our application.
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener =
        std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind TCP socket address.");
    let socket_address = listener.local_addr().unwrap();
    let server = ztp::run(listener).expect("Failed to bind address.");
    // Launch the server as a background task.
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);
    format!("http://{}", socket_address.to_string())
}

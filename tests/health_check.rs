#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app();
    // Use `reqwest` to perform HTTP requests against our app
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application in the background.
// If we fail to perform the required setup we can just panic and
// crash all the things.
fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");
    // Launch the server as a background task.
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let.
    let _ = tokio::spawn(server);
}

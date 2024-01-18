#[tokio::test]
async fn health_check_works() {
    // Arrange
    // No `.await`, no `.expect`
    spawn_app();
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
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

// No `.await` call, therefore no need for `spawn_app` to be async now.
// Since tests are running, it is not worth it to propagate errors:
// if the required setup fails, this can panic and crash.
// Launch our application in the background ~somehow~
fn spawn_app() {
    let server = zero2production::run("127.0.0.1:0").expect("Failed to bind address.");
    // Launch server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but there is no use for it here, hence the non-binding let.
    let _ = tokio::spawn(server);
}

use std::net::TcpListener;

// tokio::test is the equivalent of tokio::main
// It also spares you from having to specify the #[test] attribute.
//
// You can inspect what code gets generated using
// cargo expand --test health_check  (or name of test file)
#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    // We need to bring into scope the reqwest lib
    // to perform HHTP requests against the app
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

    // Launch App in background
    fn spawn_app() -> String {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
        let port = listener.local_addr().unwrap().port();
        let server = email_newsletter::run(listener).expect("Failed to bind address");
        let _ = tokio::spawn(server);

        format!("http://127.0.0.1:{}", port)
    }
}

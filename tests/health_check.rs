#[tokio::test]
async fn health_check_succeeds() {
    let app_url = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", app_url))
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

fn spawn_app() -> String {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind");
    let port = listener.local_addr().unwrap().port();
    println!("Running on port: {}", port);
    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_succeeds() {
    let port = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(format!(
            "http://localhost:{}/health_check",
            port.to_string()
        ))
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());
}

fn spawn_app() -> u16 {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind");
    let port = listener.local_addr().unwrap().port();
    println!("Running on port: {}", port);
    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    port
}

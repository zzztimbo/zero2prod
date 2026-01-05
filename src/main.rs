use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init();

    let listener = std::net::TcpListener::bind("127.0.0.1:8000")?;
    run(listener)?.await
}

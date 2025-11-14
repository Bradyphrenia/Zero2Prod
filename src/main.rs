use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Listener für einen festen Port erstellen
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind port 8000");

    // run liefert vermutlich ein Future oder Server, den wir awaiten
    run(listener)?.await
}

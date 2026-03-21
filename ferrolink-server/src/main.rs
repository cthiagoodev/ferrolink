use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:9000").await.unwrap();
    println!("Listening on port 9000...");

    loop {
        let (mut socket, address) = listener.accept().await.unwrap();
        println!("Accepted connection from: {address}");

        tokio::spawn(async move {
            let mut buf = vec![0u8; 1024];
            let read = socket.read(&mut buf).await.unwrap();
            let message = String::from_utf8_lossy(&buf[..read]);
            println!("Received: {message}");
            socket.write_all(b"Hello from ferrolink!\n").await.unwrap();
        });
    }
}
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:9001").await.unwrap();
    println!("Conectado ao servidor!");

    stream.write(b"Oi servidor, sou o client!").await.unwrap();

    let mut buf = vec![0u8; 1024];
    let n = stream.read(&mut buf).await.unwrap();
    let message = String::from_utf8_lossy(&buf[..n]);
    println!("Servidor respondeu: {message}");
}

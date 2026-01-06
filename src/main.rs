use leaflet_rs::client_connection::ClientConnection;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), tokio::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:25565").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            process_socket(socket).await.ok();
        });
    }
}

async fn process_socket(socket: TcpStream) -> Result<(), tokio::io::Error> {
    let mut connection = ClientConnection::new(socket);

    loop {
        connection.poll().await;
    }
}

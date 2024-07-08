mod client;
mod byte_reader;
mod byte_writer;
mod constant;
mod packet;

use tokio::net::TcpListener;
use crate::client::Client;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:1718").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let mut client = Client::from(socket);
            client.process().await;
        });
    }
}

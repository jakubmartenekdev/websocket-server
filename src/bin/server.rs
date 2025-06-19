use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};

use websockets::ThreadPool;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");
    println!("WebSocket server listening on {}", addr);

    let thread_pool= ThreadPool::new(4);
    //let test = async {};
    while let Ok((stream, _)) = listener.accept().await {
        thread_pool.execute(async {handle_connection(stream).await }).await;
    }
}

async fn handle_connection(stream: TcpStream) {
    let ws_stream = accept_async(stream).await.expect("Failed to accept connection");
    println!("New WebSocket connection");

    let (mut write, mut read) = ws_stream.split();

    if let Some(msg) = read.next().await {
        match msg {
           Ok(Message::Text(text)) => {
                println!("Received: {}", text);
                write.send(Message::Text("Ok".into())).await.unwrap();
            }
            Ok(Message::Close(_)) => {
                println!("Client closed connection");
                //break;
            }
            _ => {}
        }
    }
}

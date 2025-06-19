
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

// simulate client connecting to server
#[tokio::main]
async fn main() {
    //let url = "wss://echo.websocket.events";
    let url = "ws://localhost:3000";

    let (stream, _) = connect_async(url).await.unwrap();
    println!("Connection established");
    let (mut write, mut read) = stream.split();
    write.send(Message::Text("test message!".into())).await.unwrap();

    if let Some(msg) = read.next().await {
        println!("Received: {:?}", msg.unwrap());
    }
}
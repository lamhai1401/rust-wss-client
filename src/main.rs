use futures_util::{SinkExt, StreamExt};
use log::*;
use tokio::io::{AsyncWriteExt, Result};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

#[tokio::main]
async fn main() -> Result<()> {
    // let (mut socket, response) =
    //     connect(Url::parse("ws://127.0.0.1:8088/ws/?id=123").unwrap()).expect("Can't connect");

    let (mut socket, _) = connect_async(
        Url::parse("ws://127.0.0.1:8088/ws/?id=123").expect("Can't connect to case count URL"),
    )
    .await
    .expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (mut write, read) = socket.split();

    let msg = Message::Text(r#"["123", "event", "data", "temp"]"#.into());

    println!("sending");
    // let _ = socket.send(msg).await?;

    write.send(msg).await.unwrap();
    // loop {
    //     let msg = socket.next().await.expect("Can't fetch case count")?;
    //     println!("Received: {}", msg);
    // }

    let read_future = read.for_each(|message| async {
        let data = message.unwrap().to_string();
        // tokio::io::stdout().write(&data).await.unwrap();
        let msg = format!("received msg from server: {:?}", data);
        println!("{:}", msg);
    });

    read_future.await;

    Ok(())
}

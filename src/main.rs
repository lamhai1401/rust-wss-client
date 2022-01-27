use futures_util::{SinkExt, StreamExt};
use log::*;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Error, Message, Result},
};
use url::Url;

#[tokio::main]
async fn main() -> Result<()> {
    // let (mut socket, response) =
    //     connect(Url::parse("ws://127.0.0.1:8088/ws/?id=123").unwrap()).expect("Can't connect");

    let (mut socket, _) = connect_async(
        Url::parse("ws://127.0.0.1:8088/ws/?id=123").expect("Can't connect to case count URL"),
    )
    .await?;

    // let _ = socket.write_message(Message::Text(r#"["123", "event", "data", "temp"]"#.into()));

    let msg = Message::Text(r#"["123", "event", "data", "temp"]"#.into());
    let _ = socket.send(msg).await?;

    loop {
        let msg = socket.next().await.expect("Can't fetch case count")?;
        println!("Received: {}", msg);
    }
}

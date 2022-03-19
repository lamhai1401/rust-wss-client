use url::Url;

use futures_util::{SinkExt, StreamExt};
// use tokio::io::{AsyncWriteExt, BufReader};
// use tokio::net::TcpStream;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio_tungstenite::{connect_async, tungstenite::Message};

mod err;
use self::err::Error;

// const URL_DEFAULT: &str = "ws://127.0.0.1:8088/ws/?id=123";

// WssClient handle wss client connect
#[warn(dead_code)]
pub struct WssClient {
    url: String,
    // rx: Arc<broadcast::Receiver<String>>,
    // tx: Arc<broadcast::Sender<String>>,
    // rx: broadcast::Receiver<String>,
    tx: broadcast::Sender<String>,
}

impl WssClient {
    pub fn new(url: String) -> Self {
        let mut uri = "".to_string();
        if url == "" {
            uri = "ws://127.0.0.1:8088/ws/?id=123".to_string();
        }

        let (tx, _rx) = broadcast::channel::<String>(100);
        // let tx = Arc::new(tx);
        // let rx = Arc::new(tx.subscribe());
        WssClient { url: uri, tx: tx }
    }

    pub async fn run(&mut self) {
        let mut rx = self.tx.subscribe();
        while let Ok(msg) = rx.recv().await {
            println!("Received: {}", msg.to_string());
        }
    }

    // connect connect wss
    pub async fn connect(&mut self) -> Result<String, Error> {
        let (socket, _) =
            connect_async(Url::parse(self.url.as_str()).expect("Can't connect to case count URL"))
                .await?;
        println!("WebSocket handshake has been successfully completed");

        let (mut writer, mut reader) = socket.split();
        let tx = self.tx.clone();
        // let mut rx = tx.subscribe();

        let msg = Message::Text(r#"["123", "event", "data", "temp"]"#.into());
        writer.send(msg).await?;

        tokio::spawn(async move {
            loop {
                match reader.next().await.unwrap() {
                    Ok(msg) => match msg {
                        Message::Binary(x) => println!("Got binary {:?}", x),
                        Message::Ping(x) => {
                            println!("Received Ping {:?}", x);
                            // let _ = writer.send(Message::Pong(vec![0])).await;
                        }
                        Message::Pong(x) => {
                            println!("Received Pong {:?}", x);
                            // let _ = writer.send(Message::Ping(vec![0])).await;
                        }
                        Message::Close(_) => {
                            println!("Received close,ss client was closed");
                            break;
                        }
                        Message::Text(x) => {
                            match tx.send(x.to_string()) {
                                Err(err) => println!("{:}?", err),
                                _ => {}
                            }
                            // // TODO error here
                            // let recv = rx.recv().await.unwrap();
                        }
                        _ => return,
                    },
                    _ => {
                        println!("Ending socket reader");
                        // let _ = writer.send(Message::Close(None)).await;
                        break;
                    }
                }
            }
        });
        // .await
        // .unwrap();
        Ok("Wss Connected".to_string())
    }
}

// https://stackoverflow.com/questions/69933869/how-can-i-make-a-rust-websocket-client
// https://github.com/snapview/tokio-tungstenite/blob/master/examples/autobahn-client.rs
// https://github1s.com/bedroombuilds/python2rust/blob/main/23_websockets_client/rust/ws_client/src/main.rs

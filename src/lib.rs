use url::Url;

use futures_util::{SinkExt, StreamExt};
use tokio::io::{AsyncWriteExt, BufReader};
// use tokio::net::TcpStream;
use tokio::sync::broadcast;
use tokio_tungstenite::{connect_async, tungstenite::Message};

mod err;
use self::err::Error;

// const URL_DEFAULT: &str = "ws://127.0.0.1:8088/ws/?id=123";

// WssClient handle wss client connect
pub struct WssClient {
    url: String,
}

impl WssClient {
    pub fn new(url: String) -> Self {
        let mut uri = "".to_string();
        if url == "" {
            uri = "ws://127.0.0.1:8088/ws/?id=123".to_string();
        }
        WssClient {
            url: uri,
            // stream: None,
        }
    }

    // connect connect wss
    pub async fn connect(&mut self) -> Result<String, Error> {
        let (socket, _) =
            connect_async(Url::parse(self.url.as_str()).expect("Can't connect to case count URL"))
                .await?;
        println!("WebSocket handshake has been successfully completed");

        let (mut writer, mut reader) = socket.split();

        let (tx, _rx) = broadcast::channel::<String>(100);

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        let msg = Message::Text(r#"["123", "event", "data", "temp"]"#.into());
        writer.send(msg).await.unwrap();

        tokio::spawn(async move {
            loop {
                // tokio::select! {
                //     result = reader.next().await => {
                //         if result.unwrap() == 0 {
                //             break;
                //         }

                //         tx.send(msg.to_string()).unwrap();
                //     };

                //     recv = rx.recv().await => {
                //         let msg = recv.unwrap();
                //         println!("Received: {}", recv);
                //     };
                // };
                match reader.next().await.unwrap() {
                    Ok(msg) => match msg {
                        Message::Binary(x) => println!("Got binary {:?}", x),
                        Message::Ping(x) => println!("Ping {:?}", x),
                        Message::Pong(x) => println!("Pong {:?}", x),
                        Message::Close(_) => {
                            println!("Wss client was closed");
                            return;
                        }
                        Message::Text(x) => {
                            tx.send(x.to_string()).unwrap();
                            let recv = rx.recv().await.unwrap();
                            println!("Received: {}", recv);
                        }
                        _ => {}
                    },
                    _ => {
                        println!("Ending socket reader");
                        return;
                    }
                }
            }
        })
        .await
        .unwrap();
        Ok("Wss Disconnected".to_string())
    }
}

// https://stackoverflow.com/questions/69933869/how-can-i-make-a-rust-websocket-client
// https://github.com/snapview/tokio-tungstenite/blob/master/examples/autobahn-client.rs

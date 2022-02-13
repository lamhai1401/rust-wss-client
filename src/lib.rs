use url::Url;

use futures_util::{SinkExt, StreamExt};
use std::net::TcpStream;
use tokio::io::AsyncWriteExt;
use tokio_tungstenite::connect_async;

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
        let (mut socket, _) =
            connect_async(Url::parse(self.url.as_str()).expect("Can't connect to case count URL"))
                .await?;
        println!("WebSocket handshake has been successfully completed");

        let (mut writer, mut reader) = socket.split();

        Ok("Wss Connected".to_string())
    }
}

// https://stackoverflow.com/questions/69933869/how-can-i-make-a-rust-websocket-client
// https://github.com/snapview/tokio-tungstenite/blob/master/examples/autobahn-client.rs

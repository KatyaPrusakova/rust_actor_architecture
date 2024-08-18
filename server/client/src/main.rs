use tokio::net::TcpStream;
use tokio_util::codec::{BytesCodec, Decoder};
use std::error::Error;
use serde::{Serialize, Deserialize};
use bincode;
use futures::sink::SinkExt;
use futures::StreamExt;
use bytes::Bytes;
use crate::types::{Body, Header, HttpFrame};

mod types;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let stream = TcpStream::connect("127.0.0.1:8080").await?;
    let mut framed = BytesCodec::new().framed(stream);

    let message = HttpFrame {
        header: Header { method: "POST".to_string(), uri: "www.weather.com/".to_string() },
        body: Body { chat_id: "WTF".to_string(), message_type: types::MessageType::INPUT, single_data: Some("WTF".to_string()) },
    };

    let message_bin = bincode::serialize(&message).unwrap();
    let sending_message = Bytes::from(message_bin);
    framed.send(sending_message).await.unwrap();

    let message = framed.next().await.unwrap().unwrap();
    let message = bincode::deserialize::<HttpFrame>(&message).unwrap();
    println!("{:?}", message);

    Ok(())
}

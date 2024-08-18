use tokio::net::TcpListener;
use bincode;
use tokio_util::codec::{BytesCodec, Decoder};
use futures::sink::SinkExt;
use futures::StreamExt;
use bytes::{Bytes};
use crate::types::HttpFrame;

mod types;



#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080".to_string();

    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("listening on {}", addr);

    loop {
        match listener.accept().await {
            Ok((socket, _)) => {
                tokio::spawn(async move {
                    let mut framed = BytesCodec::new().framed(socket);
                    while let Some(message_result) = framed.next().await {
                        match message_result {
                            Ok(bytes) => {
                                match bincode::deserialize::<HttpFrame>(&bytes) {
                                    Ok(message) => {
                                        
                                        
                                        println!("{:?}", message);
                                    },
                                    Err(e) => {
                                        eprintln!("Failed to deserialize message: {:?}", e);
                                        break;
                                    }
                                }
                            },
                            Err(e) => {
                                eprintln!("Error reading from socket: {:?}", e);
                                break;
                            }
                        }
                    }
                });
            },
            Err(e) => eprintln!("Failed to accept connection: {:?}", e),
        }
    }
}
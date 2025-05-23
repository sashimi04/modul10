use futures_util::{SinkExt, StreamExt};
use std::{error::Error, net::SocketAddr};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{channel, Sender};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut rx = bcast_tx.subscribe();

    loop {
        tokio::select! {
            msg = ws_stream.next() => {
                match msg {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                            // ✅ Tambahkan informasi pengirim (IP:Port)
                            let send_msg = format!("From {addr}: {text}");
                            println!("{send_msg}");
                            let _ = bcast_tx.send(send_msg);
                        }
                    }
                    Some(Err(e)) => {
                        eprintln!("error receiving ws message: {e}");
                        break;
                    }
                    None => break,
                }
            }
            result = rx.recv() => {
                if let Ok(msg) = result {
                    if let Err(e) = ws_stream.send(Message::text(msg)).await {
                        eprintln!("error sending ws message: {e}");
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);
    let listener = TcpListener::bind("127.0.0.1:8081").await?;

    println!("listening on port 8081");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr:?}");
        let bcast_tx = bcast_tx.clone();

        tokio::spawn(async move {
            let result = async {
                let (_req, ws_stream) = ServerBuilder::new()
                    .accept(socket)
                    .await
                    .expect("failed to accept websocket");

                handle_connection(addr, ws_stream, bcast_tx).await
            };

            if let Err(e) = result.await {
                eprintln!("error handling connection: {e}");
            }
        });
    }
}

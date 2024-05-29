use futures_util::{future, pin_mut, StreamExt};
use rust_websockets::models::websocket::{CombinedStreamEvent, WebSocketEventResponse};
use tokio::io::AsyncReadExt;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

#[tokio::main]
async fn main() {
    let url = url::Url::parse(
        "wss://stream.binance.com:9443/stream?streams=btcusdt@aggTrade/btcusdt@trade/btcusdt@kline",
    )
    .unwrap();

    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    tokio::spawn(read_stdin(stdin_tx));

    let (ws_stream, _response) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (write, read) = ws_stream.split();
    let stdin_to_ws = stdin_rx.map(Ok).forward(write);
    let ws_to_stdout = {
        read.for_each(|message| async {
            match message {
                Ok(msg) => {
                    let string = msg.to_text().unwrap();
                    if let Ok(response) =
                        serde_json::from_str::<CombinedStreamEvent<WebSocketEventResponse>>(&string)
                    {
                        match response.data {
                            WebSocketEventResponse::AggregateTrade(val) => {
                                println!("got aggragate trade")
                            }
                            WebSocketEventResponse::Trade(_) => {
                                println!("got trade")
                            }
                            WebSocketEventResponse::Kline(_) => {
                                println!("got Kline")
                            }
                        }
                    } else {
                        println!("error deserialising message {}", msg)
                    }
                }
                Err(e) => {
                    eprintln!("Error receiving message: {:?}", e);
                }
            }
        })
    };

    pin_mut!(stdin_to_ws, ws_to_stdout);
    future::select(stdin_to_ws, ws_to_stdout).await;
}
// Our helper method which will read data from stdin and send it along the
// sender provided.
async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) {
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buf = vec![0; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);
        tx.unbounded_send(Message::Text(String::from_utf8_lossy(&buf).to_string()))
            .unwrap();
    }
}

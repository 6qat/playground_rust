use mini_redis::client;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    // https://tokio.rs/tokio/tutorial/channels
    // mpsc: multi-producer, single-consumer channel. Many values can be sent.
    // oneshot: single-producer, single consumer channel. A single value can be sent.
    // broadcast: multi-producer, multi-consumer. Many values can be sent. Each receiver sees every value.
    // watch: single-producer, multi-consumer. Many values can be sent, but no history is kept. Receivers only see the most recent value.
    let (tx1, mut rx) = mpsc::channel(32);
    let tx2 = tx1.clone();

    // tx.send("sending from first handle").await;
    // tx2.send("sending from second handle").await;

    tokio::spawn(async move {
        tx1.send("sending from first handle").await;
    });

    tokio::spawn(async move {
        tx2.send("sending from second handle").await;
    });

    while let Some(message) = rx.recv().await {
        println!("GOT = {}", message);
    }
}

use bytes::Bytes;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
    },
    Set {
        key: String,
        val: Bytes,
    },
}


use mini_redis::{Connection, Frame};
#[allow(unused_assignments)]
#[allow(unused_variables)]
#[allow(dead_code)]
use tokio::net::{TcpListener, TcpStream};
use bytes::Bytes;
use std::collections::HashMap;

// std::sync::Mutex and not tokio::sync::Mutex
// consider using parking_lot::Mutex as a faster alternative to std::sync::Mutex
use std::sync::{Arc, Mutex};
// use parking_lot::Mutex;

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

// https://tokio.rs/tokio/tutorial/spawning
//
// *** When we say that a value is 'static, all that means is that it
// would not be incorrect to keep that value around forever.
// This is important because the compiler is unable to reason about
// how long a newly spawned task stays around, so the only way it can be
// sure that the task doesn't live too long is to make sure it may live forever.

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Listening");
    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone();

        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};

    // A hashmap is used to store data

    // The `Connection` lets us read/write redis **frames** instead of
    // byte streams. The `Connection` type is defined by mini-redis.
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    // `Frame::Bulk` expects data to be of type `Bytes`. This
                    // type will be covered later in the tutorial. For now,
                    // `&Vec<u8>` is converted to `Bytes` using `into()`.
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // Write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}

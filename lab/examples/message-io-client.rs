use std::time::Duration;

use message_io::network::{NetEvent, Transport};
use message_io::node::{self, NodeEvent};

enum Signal {
    Greet,
    // Any other app event here.
}

fn main() {
    let (handler, listener) = node::split();

    // You can change the transport to Udp or Ws (WebSocket).
    let (server, _) = handler
        .network()
        .connect(Transport::FramedTcp, "127.0.0.1:3042")
        .unwrap();

    listener.for_each(move |event| match event {
        NodeEvent::Network(net_event) => match net_event {
            NetEvent::Connected(_endpoint, _ok) => handler.signals().send(Signal::Greet),
            NetEvent::Accepted(_, _) => unreachable!(), // Only generated by listening
            NetEvent::Message(_endpoint, data) => {
                println!("Received: {}", String::from_utf8_lossy(data));
            }
            NetEvent::Disconnected(_endpoint) => (),
        },
        NodeEvent::Signal(signal) => match signal {
            Signal::Greet => {
                // computed every second
                handler.network().send(server, "Hello server!".as_bytes());
                handler
                    .signals()
                    .send_with_timer(Signal::Greet, Duration::from_secs(1));
            }
        },
    });
}

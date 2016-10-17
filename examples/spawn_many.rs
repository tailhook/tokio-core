//! An echo server that just writes back everything that's written to it.

extern crate env_logger;
extern crate futures;
extern crate tokio_core;

use std::env;
use std::net::SocketAddr;

use futures::{Future, empty};
use futures::stream::Stream;
use tokio_core::io::{copy, Io};
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;

fn main() {
    env_logger::init().unwrap();
    let addr = env::args().nth(1).unwrap_or("127.0.0.1:8080".to_string());
    let addr = addr.parse::<SocketAddr>().unwrap();

    // Create the event loop that will drive this server
    let mut l = Core::new().unwrap();
    let handle = l.handle();

    // Create a TCP listener which will listen for incoming connections
    let socket = TcpListener::bind(&addr, &handle).unwrap();

    // Once we've got the TCP listener, inform that we have it
    println!("Listening on: {}", addr);

    // Pull out the stream of incoming connections and then for each new
    // one spin up a new task copying data.
    //
    // We use the `io::copy` future to copy all data from the
    // reading half onto the writing half.
    handle.spawn_many(1, socket.incoming().map(move |(socket, addr)| {
        futures::lazy(|| futures::finished(socket.split()))
        .and_then(|(reader, writer)| copy(reader, writer))
        .map(move |amt| {
            println!("wrote {} bytes to {}", amt, addr)
            })
        .map_err(|e| {
            panic!("error: {}", e);
            })
    }).map_err(|_| ()));
    l.run(empty::<(), ()>()).unwrap();
}

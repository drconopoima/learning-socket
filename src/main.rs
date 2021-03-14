use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixListener, UnixStream};
use std::thread;

fn handle_client(stream: UnixStream) {
    // Add Buffer to stream
    let stream = BufReader::new(stream);
    for line in stream.lines() {
        println!(
            "{}",
            line.unwrap_or_else(|error| {
                panic!(format!(
                    "learning-socket.main.handle_client: Could not read line from stream {:?}",
                    error
                ))
            })
        )
    }
}

fn main() {
    let listener = UnixListener::bind("/tmp/rust-uds.sock").unwrap_or_else(|error| {
        panic!(format!(
            "learning-socket.main: Could not bind to socket. {:?}",
            error
        ))
    });
    for stream in listener.incoming() {
        stream.map_or_else(
            |error| {
                println!(
                    "{:?}",
                    format!(
                        "learning-socket.main: Could not listen socket incoming {:?}",
                        error
                    )
                );
            },
            |stream| {
                println!("Got connection request");
                thread::spawn(|| handle_client(stream));
            },
        )
    }
}

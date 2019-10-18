use std::net::{Ipv4Addr};

use comms_lib::{Client, ClientCommand, ServerCommand};

fn main() {
    let mut client = Client::connect(Ipv4Addr::LOCALHOST)
        .expect("Failed to connect the client");
    client.send(ClientCommand::Hello)
        .expect("Failed to send the client hello");
    match client.recv().expect("Failed to receive the server's response") {
        ServerCommand::Hello => {
            println!("Got a hello from the server");
        },
    }
}

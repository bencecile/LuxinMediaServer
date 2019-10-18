use std::net::{Ipv4Addr};

use comms_lib::{Client, ClientCommand, ServerCommand};

fn main() {
    comms_lib::init()
        .expect("Failed to initialize the client's communication library");

    let mut client = Client::connect(Ipv4Addr::LOCALHOST)
        .expect("Failed to connect the client");
    client.send(ClientCommand::RequestTestMessage)
        .expect("Failed to request a test message");
    match client.recv().expect("Failed to receive the server's response") {
        ServerCommand::TestMessage(test_message) => {
            println!("Got a message from the server: {}", test_message);
        },
        _ => panic!("Failed to get a test message from the server"),
    }

    client.send(ClientCommand::Exit)
        .expect("Failed to send an exit");
    match client.recv().expect("Failed to receive the server's response to Exit") {
        ServerCommand::Ok => {
            println!("Got an OK from the server for exit");
        },
        _ => panic!("Failed to get an OK for the exit"),
    }
}

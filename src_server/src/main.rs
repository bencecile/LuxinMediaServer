use comms_lib::{ClientCommand, Server, ServerCommand};

fn main() {
    comms_lib::init()
        .expect("Failed to initialize the server's communication library");

    let server = Server::start()
        .expect("Failed to start the server");
    let mut server_conn = server.accept()
        .expect("Failed to accept a new connection");
    // TODO Do this in a good async context
    loop {
        match server_conn.recv().expect("Failed to receive from the client") {
            ClientCommand::RequestTestMessage => {
                println!("Sending a test message to the client");
                server_conn.send(ServerCommand::TestMessage(
                    "Test message 123@$![]あいうエオ日本語".to_string()
                )).expect("Failed to send the test message");
            },
            ClientCommand::Exit => {
                println!("Sending an OK to the client for exit");
                server_conn.send(ServerCommand::Ok)
                    .expect("Failed to send the OK for exit");
                break;
            },
        }
    }
}

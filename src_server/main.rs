use comms_lib::{ClientCommand, Server, ServerCommand};

fn main() {
    let server = Server::start()
        .expect("Failed to start the server");
    let mut server_conn = server.accept()
        .expect("Failed to accept a new connection");
    match server_conn.recv().expect("Failed to receive from the client") {
        ClientCommand::Hello => {
            println!("Got a hello from the client");
            server_conn.send(ServerCommand::Hello)
                .expect("Failed to send the hello command back");
        }
    }
}

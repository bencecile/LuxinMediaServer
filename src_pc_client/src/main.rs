use std::net::{Ipv4Addr};
use comms_lib::{Client, ClientCommand, ServerCommand};
use client_lib::{App};
use ui_lib::*;

fn main() {
    comms_lib::init().expect("Failed to initialize the client's communication library");

    // TODO Get rid of window decorations, instead doing your own thing
    let app = App::new(ClientData::default(), HomeState::default());
    let (ui_context, event_loop) = UIContext::new(WindowCreateOptions::default())
        .expect("Failed to create a UI Context");
    app.run(ui_context, event_loop);
}

#[derive(Default)]
struct ClientData {
    server_address: Option<Ipv4Addr>,
    client: Option<Client>,
}
impl ClientData {
    fn set_server_address(&mut self, server_address: Ipv4Addr) {
        // Ipv4Addr::LOCALHOST
        self.server_address = Some(server_address);
    }

    fn try_connect(&mut self) {
        if let Some(server_address) = self.server_address {
            if let Ok(client) = Client::connect() {
                self.client = Some(client);
            } else {
                println!("Failed to connect to {:?}", self.server_address);
            }
        } else {
            println!("We don't have a server address");
        }
    }
}

#[derive(Default)]
struct HomeState {
    test_message: Option<String>,
}
impl AppState for HomeState {
    type AppData = ClientData;
    type NodeID = HomeElements;

    fn layout(&mut self, client_data: &ClientData) -> RootNode<HomeElements> {
        let label = Label::new(format!("{:?}", &self.test_message));
        let button = Button::with_label("Fetch message");
            // .with_callback(On::MouseUp, fetch_message_callback);

        RootNode::new()
            .with_child(label)
            .with_child(button)
    }
}

    // fn fetch_test_message(&mut self) {
    //     self.client.send(ClientCommand::RequestTestMessage)
    //         .expect("Failed to request a test message");
    //     match self.client.recv().expect("Failed to receive the server's response") {
    //         ServerCommand::TestMessage(test_message) => {
    //             self.test_message = Some(test_message);
    //         },
    //         _ => panic!("Failed to get a test message from the server"),
    //     }
    // }

    // fn exit(&mut self) {
    //     self.client.send(ClientCommand::Exit)
    //         .expect("Failed to send an exit");
    //     match self.client.recv().expect("Failed to receive the server's response to Exit") {
    //         ServerCommand::Ok => {
    //             println!("Got an OK from the server for exit");
    //         },
    //         _ => panic!("Failed to get an OK for the exit"),
    //     }
    // }

// fn fetch_message_callback(event: CallbackInfo<ClientApplication>) -> UpdateScreen {
//     event.state.data.fetch_test_message();
//     Redraw
// }

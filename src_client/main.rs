use std::net::{Ipv4Addr};

use amethyst::{
    Logger, LoggerConfig, LogLevelFilter, StdoutLog,
    core::transform::{TransformBundle},
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{
        plugins::{RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    window::{DisplayConfig},
};

use comms_lib::{Client, ClientCommand, ServerCommand};

fn main() {
    comms_lib::init()
        .expect("Failed to initialize the client's communication library");
    Logger::from_config(LoggerConfig {
        stdout: StdoutLog::Colored,
        level_filter: LogLevelFilter::Info,
        // TODO Write to a log file instead of stdout when not debugging
        log_file: None,
        allow_env_override: false,
        log_gfx_device_level: None,
    }).level_for("gfx_backend_vulkan", LogLevelFilter::Off)
        .start();

    let mut display_config = DisplayConfig::default();
    display_config.title = "Luxin Media Client".to_string();
    display_config.dimensions = Some( (1024, 720) );

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())
            .expect("Failed to add the transform bundle")
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config(display_config)
                        .with_clear([0.34, 0.36, 0.52, 1.0])
                )
        ).expect("Failed to add the rendering bundle");

    let mut game = Application::new("./", MyState::new(), game_data)
        .expect("Failed to make the application");
    game.run();
}

pub struct MyState {
    client: Client,
    test_message: Option<String>,
}
impl MyState {
    fn new() -> MyState {
        let client = Client::connect(Ipv4Addr::LOCALHOST)
            .expect("Failed to connect the client");
        MyState {
            client,
            test_message: None,
        }
    }

    fn exit(&mut self) {
        self.client.send(ClientCommand::Exit)
            .expect("Failed to send an exit");
        match self.client.recv().expect("Failed to receive the server's response to Exit") {
            ServerCommand::Ok => {
                println!("Got an OK from the server for exit");
            },
            _ => panic!("Failed to get an OK for the exit"),
        }
    }
}
impl SimpleState for MyState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        self.client.send(ClientCommand::RequestTestMessage)
            .expect("Failed to request a test message");
        match self.client.recv().expect("Failed to receive the server's response") {
            ServerCommand::TestMessage(test_message) => {
                self.test_message = Some(test_message);
            },
            _ => panic!("Failed to get a test message from the server"),
        }
    }

    fn handle_event(&mut self, mut _data: StateData<'_, GameData<'_, '_>>, event: StateEvent)
    -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                self.exit();
                return Trans::Quit;
            }

            if let Some(event) = get_key(&event) {
                println!("handling key event: {:?}", event);
            }
        }
        Trans::None
    }
}

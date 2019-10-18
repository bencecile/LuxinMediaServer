use std::io::{BufReader, BufWriter, Error as IOError};
use std::io::prelude::*;
use std::net::{Ipv4Addr, TcpListener, TcpStream};

use bincode::{Error as BincodeError};
use serde::{Deserialize, Serialize};
use serde::de::{DeserializeOwned};

#[derive(Deserialize, Serialize)]
pub enum ClientCommand {
    Hello,
}
#[derive(Deserialize, Serialize)]
pub enum ServerCommand {
    Hello,
}

pub type CommsResult<T> = Result<T, CommsError>;
#[derive(Debug)]
pub enum CommsError {
    IO(IOError),
    Bincode(BincodeError),
}
impl From<IOError> for CommsError {
    fn from(error: IOError) -> Self { Self::IO(error) }
}
impl From<BincodeError> for CommsError {
    fn from(error: BincodeError) -> Self { Self::Bincode(error) }
}

// TODO Make an init function for library consumers to call first

const SERVER_PORT: u16 = 50025;

pub struct Client {
    stream: TcpStream,
}
impl Client {
    pub fn connect(addr: Ipv4Addr) -> CommsResult<Client> {
        Ok(Client {
            stream: TcpStream::connect( (addr, SERVER_PORT) )?,
        })
    }

    pub fn send(&mut self, command: ClientCommand) -> CommsResult<()> {
        write_serialize(&mut self.stream, &command)
    }
    pub fn recv(&mut self) -> CommsResult<ServerCommand> {
        read_deserialize(&mut self.stream)
    }
}

pub struct Server {
    listener: TcpListener,
}
impl Server {
    pub fn start() -> CommsResult<Server> {
        Ok(Server {
            listener: TcpListener::bind( (Ipv4Addr::UNSPECIFIED, SERVER_PORT) )?,
        })
    }

    pub fn accept(&self) -> CommsResult<ServerConnection> {
        let (stream, _addr) = self.listener.accept()?;
        Ok(ServerConnection {
            stream,
        })
    }
}

pub struct ServerConnection {
    stream: TcpStream,
}
impl ServerConnection {
    pub fn send(&mut self, command: ServerCommand) -> CommsResult<()> {
        write_serialize(&mut self.stream, &command)
    }
    pub fn recv(&mut self) -> CommsResult<ClientCommand> {
        read_deserialize(&mut self.stream)
    }
}

fn write_serialize(writer: &mut impl Write, to_serialize: &impl Serialize) -> CommsResult<()> {
    bincode::serialize_into(BufWriter::new(writer), to_serialize)?;
    Ok(())
}
fn read_deserialize<T: DeserializeOwned>(reader: &mut impl Read) -> CommsResult<T> {
    Ok(bincode::deserialize_from(BufReader::new(reader))?)
}

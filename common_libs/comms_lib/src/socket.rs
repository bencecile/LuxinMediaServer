use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use std::net::{Ipv4Addr, TcpListener, TcpStream};

use serde::{Deserialize, Serialize};
use serde::de::{DeserializeOwned};
use sodiumoxide::crypto::box_::{self, Nonce, PublicKey, SecretKey};

use crate::{CommsError, CommsResult, CryptoError};

const SERVER_PORT: u16 = 50025;

#[derive(Deserialize, Serialize)]
enum HandshakeCommand {
    Ok,
    SendPublicKey(PublicKey),
}
impl HandshakeCommand {
    fn got_ok(&self) -> bool {
        match self {
            Self::Ok => true,
            _ => false,
        }
    }
}

struct HandshakeClient {
    stream: TcpStream,
}
impl HandshakeClient {
    fn new(stream: TcpStream) -> HandshakeClient {
        HandshakeClient {
            stream,
        }
    }

    fn perform_handshake(mut self, pub_key: PublicKey) -> CommsResult<(PublicKey, TcpStream)> {
        self.send(HandshakeCommand::SendPublicKey(pub_key))?;
        let other_public_key = match self.recv()? {
            HandshakeCommand::SendPublicKey(pub_key) => pub_key,
            _ => return Err(CommsError::Crypto(CryptoError::BadHandshake(
                "Failed to get the other client's public key first".to_string()
            ))),
        };
        self.send(HandshakeCommand::Ok)?;
        if !self.recv()?.got_ok() {
            return Err(CommsError::Crypto(CryptoError::BadHandshake(
                "Failed to get an OK from the other client".to_string()
            )));
        }

        Ok( (other_public_key, self.stream) )
    }

    fn send(&mut self, command: HandshakeCommand) -> CommsResult<()> {
        bincode::serialize_into(BufWriter::new(&mut self.stream), &command)?;
        Ok(())
    }
    fn recv(&mut self) -> CommsResult<HandshakeCommand> {
        Ok(bincode::deserialize_from(BufReader::new(&mut self.stream))?)
    }
}

#[derive(Deserialize, Serialize)]
pub enum ClientCommand {
    Hello,
}
#[derive(Deserialize, Serialize)]
pub enum ServerCommand {
    Hello,
}

pub struct Client {
    stream: TcpStream,
    private_key: SecretKey,
    server_public_key: PublicKey,
}
impl Client {
    pub fn connect(addr: Ipv4Addr) -> CommsResult<Client> {
        let (pub_key, private_key) = box_::gen_keypair();
        let (server_public_key, stream) = HandshakeClient::new(
            TcpStream::connect( (addr, SERVER_PORT) )?
        ).perform_handshake(pub_key)?;

        Ok(Client {
            stream,
            private_key,
            server_public_key,
        })
    }

    pub fn send(&mut self, command: ClientCommand) -> CommsResult<()> {
        write_encrypted_command(
            &mut self.stream, command,
            &self.private_key, &self.server_public_key
        )
    }
    pub fn recv(&mut self) -> CommsResult<ServerCommand> {
        read_encrypted_command(
            &mut self.stream,
            &self.private_key, &self.server_public_key
        )
    }
}

pub struct Server {
    listener: TcpListener,
    private_key: SecretKey,
}
impl Server {
    pub fn start() -> CommsResult<Server> {
        let (_pub_key, private_key) = box_::gen_keypair();
        Ok(Server {
            listener: TcpListener::bind( (Ipv4Addr::UNSPECIFIED, SERVER_PORT) )?,
            private_key,
        })
    }

    pub fn accept(&self) -> CommsResult<ServerConnection> {
        let (stream, _addr) = self.listener.accept()?;
        let (client_public_key, stream) = HandshakeClient::new(stream)
            .perform_handshake(self.private_key.public_key())?;

        Ok(ServerConnection {
            stream,
            private_key: self.private_key.clone(),
            client_public_key,
        })
    }
}

pub struct ServerConnection {
    stream: TcpStream,
    private_key: SecretKey,
    client_public_key: PublicKey,
}
impl ServerConnection {
    pub fn send(&mut self, command: ServerCommand) -> CommsResult<()> {
        write_encrypted_command(
            &mut self.stream, command,
            &self.private_key, &self.client_public_key
        )
    }
    pub fn recv(&mut self) -> CommsResult<ClientCommand> {
        read_encrypted_command(
            &mut self.stream,
            &self.private_key, &self.client_public_key
        )
    }
}

struct EncMessage {
    // We need the nonce to be unique between each message
    nonce: Nonce,
    // The encrypted bytes that we sent in the message
    enc_body: Vec<u8>,
}
impl EncMessage {
    fn new(body: impl Serialize,
    private_key: &SecretKey, other_public_key: &PublicKey) -> CommsResult<EncMessage> {
        let body_bytes = bincode::serialize(&body)?;

        let nonce = box_::gen_nonce();
        let enc_body = box_::seal(&body_bytes, &nonce, other_public_key, private_key);

        Ok(EncMessage {
            nonce,
            enc_body,
        })
    }

    fn from_reader(mut reader: impl Read) -> CommsResult<EncMessage> {
        let mut length_bytes = [0_u8; 8];
        reader.read_exact(&mut length_bytes)?;
        let enc_body_length = u64::from_le_bytes(length_bytes) as usize;

        let nonce: Nonce = bincode::deserialize_from(&mut reader)?;

        let mut enc_body = vec![0_u8; enc_body_length];
        reader.read_exact(&mut enc_body)?;

        Ok(EncMessage {
            nonce,
            enc_body,
        })
    }
    fn decrypt<T: DeserializeOwned>(&self,
    private_key: &SecretKey, other_public_key: &PublicKey) -> CommsResult<T> {
        let body_bytes = box_::open(&self.enc_body, &self.nonce, other_public_key, private_key)
            .map_err(|_| CommsError::Crypto(CryptoError::UnencryptFailed))?;
        Ok(bincode::deserialize(&body_bytes)?)
    }

    fn into_writer(&self, mut writer: impl Write) -> CommsResult<()> {
        let enc_body_length = self.enc_body.len() as u64;
        writer.write_all(&enc_body_length.to_le_bytes())?;
        bincode::serialize_into(&mut writer, &self.nonce)?;
        writer.write_all(&self.enc_body)?;
        Ok(())
    }
}

fn write_encrypted_command(writer: &mut impl Write, command: impl Serialize,
private_key: &SecretKey, other_public_key: &PublicKey) -> CommsResult<()> {
    let enc_message = EncMessage::new(command, private_key, other_public_key)?;
    enc_message.into_writer(BufWriter::new(writer))
}

fn read_encrypted_command<T: DeserializeOwned>(reader: &mut impl Read,
private_key: &SecretKey, other_public_key: &PublicKey) -> CommsResult<T> {
    let enc_message = EncMessage::from_reader(BufReader::new(reader))?;
    enc_message.decrypt(private_key, other_public_key)
}

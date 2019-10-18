use std::io::{BufReader, BufWriter};
use std::net::{TcpStream};

use serde::{Deserialize, Serialize};
use sodiumoxide::crypto::box_::{PublicKey};

use crate::{CommsResult, CommsError, CryptoError};

#[derive(Deserialize, Serialize)]
pub enum HandshakeCommand {
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

pub struct HandshakeClient {
    stream: TcpStream,
}
impl HandshakeClient {
    pub fn new(stream: TcpStream) -> HandshakeClient {
        HandshakeClient {
            stream,
        }
    }

    pub fn perform_handshake(mut self, pub_key: PublicKey) -> CommsResult<(PublicKey, TcpStream)> {
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

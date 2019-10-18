mod socket;
pub use self::socket::{
    Client, Server, ServerConnection,
    ClientCommand, ServerCommand,
};

use std::io::{Error as IOError};

use bincode::{Error as BincodeError};

pub type CommsResult<T> = Result<T, CommsError>;
#[derive(Debug)]
pub enum CommsError {
    Bincode(BincodeError),
    Crypto(CryptoError),
    IO(IOError),
}
impl From<IOError> for CommsError {
    fn from(error: IOError) -> Self { Self::IO(error) }
}
impl From<BincodeError> for CommsError {
    fn from(error: BincodeError) -> Self { Self::Bincode(error) }
}

#[derive(Debug)]
pub enum CryptoError {
    BadHandshake(String),
    UnencryptFailed,
    InitFailed,
}

pub fn init() -> CommsResult<()> {
    sodiumoxide::init()
        .map_err(|_| CommsError::Crypto(CryptoError::InitFailed))
}

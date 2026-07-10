use crate::Utf8Error;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum BufferError {
    TooManyBytesRead,
}

impl From<std::io::Error> for ClientError {
    fn from(error: std::io::Error) -> Self {
        let _ = error;
        ClientError::Io
    }
}

impl From<FromUtf8Error> for ClientError {
    fn from(error: FromUtf8Error) -> ClientError {
        let _ = error;
        ClientError::StringConversion
    }
}

impl From<std::str::Utf8Error> for ClientError {
    fn from(error: Utf8Error) -> ClientError {
        let _ = error;
        ClientError::StringConversion
    }
}
#[derive(Debug)]
pub enum ClientError {
    TcpStreamRead(BufferError),
    Io,
    StringConversion,
    RequestStringError(RequestLineError),
    UnsupportedRequest,
}

#[derive(Debug)]
pub enum RequestLineError {
    SlashParsing,
}

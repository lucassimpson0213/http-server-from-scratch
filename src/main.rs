#![deny(warnings)]
#![deny(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]
#![deny(clippy::dbg_macro)]
#![deny(unsafe_code)]
use core::str::Utf8Error;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    string::FromUtf8Error,
};
fn main() {
    println!("hello");

    let val = bind_port();
    println!("{:?}", val);
}

enum BufferError {
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

impl From<Utf8Error> for ClientError {
    fn from(error: Utf8Error) -> ClientError {
        let _ = error;
        ClientError::StringConversion
    }
}
enum ClientError {
    TcpStreamRead(BufferError),
    Io,
    StringConversion,
}

fn is_get_request(buf: &[u8]) -> Result<bool, Utf8Error> {
    //does not require a heap allocation, this is used instead
    // -- of String::from_utf8 because that would consume the buffer and use the underlying vec8
    // -- we need to preserve the vec for further consumption??
    let request_str = str::from_utf8(buf)?;

    if request_str.contains("GET") {
        Ok(true)
    } else {
        Ok(false)
    }
}

fn parse_request_target(buf: &[u8]) -> Result<(), std::str::FromUtf8Error> {
    //the bytes that the buffer returns may or may not actually be text all the time
    //let actual_string = std::str::from_utf8(buf)?;
    //don't make a habit of converting everything to text
    //
    //
    //

    //now convert the buffer into ut8f text
    //at first just take a view into the u8 vec and borrow it using a byte slice
    //this avoids having to allocate a string or convert way too early and pass it around

    return Ok(());
}
fn handle_client(stream: TcpStream, _listener: &TcpListener) -> Result<(), ClientError> {
    println!("{:?}", stream);

    let mut owned_stream = stream;
    let mut buf: Vec<u8> = vec![0u8; 16834];

    let amt_bytes = owned_stream.read(&mut buf)?;

    debug_assert!(amt_bytes <= 16834);

    if amt_bytes > 16834 {
        let too_many_bytes_read = BufferError::TooManyBytesRead;
        return Err(ClientError::TcpStreamRead(too_many_bytes_read));
    }

    if is_get_request(&buf[..amt_bytes])? {
        parse_request_target(&buf[..amt_bytes]?);
    }
    //implement from trait for Utf8Error to ClientError
    //you cannot use ? because it implicitly tries to cast utf8 error to client error

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    let response404 = "HTTP/1.1 404 Not Found\r\n\r\n";

    owned_stream.write_all(response.as_bytes())?;
    owned_stream.write_all(response404.as_bytes())?;

    //read_exact or maybe another method that chunks each section that points to the beginning of
    //the section
    // let bytes_read = &owned_stream.read_exact(owned_buf)?;
    //
    //
    //:#![warn()]

    Ok(())
}
fn bind_port() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:4221")?;

    for stream in listener.incoming() {
        let _ = handle_client(stream?, &listener);
    }

    Ok(())
}

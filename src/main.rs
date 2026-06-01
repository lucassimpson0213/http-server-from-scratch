use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    str::Utf8Error,
};

use std::io::Error as GenericError;
fn main() {
    println!("hello");

    let val = bind_port();
    println!("{:?}", val);
}
enum StreamError {
    Fail,
    Success,
}

enum BufferError {
    TooManyBytesRead,
}

impl From<std::io::Error> for ClientError {
    fn from(error: std::io::Error) -> Self {
        ClientError::IoError(error)
    }
}
enum ClientError {
    TcpStreamReadError(BufferError),
    IoError(std::io::Error),
}

fn is_get_request(buf: &Vec<u8>) -> Result<bool, Utf8Error> {
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
fn handle_client(stream: TcpStream, listener: &TcpListener) -> Result<(), ClientError> {
    println!("{:?}", stream);

    let mut owned_stream = stream;
    let mut buf: Vec<u8> = vec![0u8; 16834];

    let amt_bytes = owned_stream.read(&mut buf)?;

    debug_assert!(amt_bytes <= 16834);

    if amt_bytes > 16834 {
        let too_many_bytes_read = BufferError::TooManyBytesRead;
        return Err(ClientError::TcpStreamReadError(
            BufferError::TooManyBytesRead,
        ));
    }

    let string = String::from_utf8(buf);

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
        handle_client(stream?, &listener);
    }

    Ok(())
}

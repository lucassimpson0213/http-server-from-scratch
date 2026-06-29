#![deny(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]
#![deny(clippy::dbg_macro)]
#![deny(unsafe_code)]
use core::str::Utf8Error;
use std::{
    error::Error,
    i32,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    string::FromUtf8Error,
};

//TODO make a test script using curl localhost:4221/abcsad
fn main() {
    //println!("hello");

    let hello = bind_port();
}

#[derive(Debug)]
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

impl From<std::str::Utf8Error> for ClientError {
    fn from(error: Utf8Error) -> ClientError {
        let _ = error;
        ClientError::StringConversion
    }
}
#[derive(Debug)]
enum ClientError {
    TcpStreamRead(BufferError),
    Io,
    StringConversion,
    RequestLineError(RequestLineError),
}

#[derive(Debug)]
enum RequestLineError {
    SpaceParsing,
    SlashParsing,
    InvalidRange,
}

fn is_get_request(buf: &[u8]) -> Result<bool, Utf8Error> {
    //does not require a heap allocation, this is used instead
    // -- of String::from_utf8 because that would consume the buffer and use the underlying vec8
    // -- we need to preserve the vec for further consumption??
    let request_str = str::from_utf8(buf)?;
    //parse request target here and return differing response.
    if request_str.contains("GET") {
        Ok(true)
    } else {
        Ok(false)
    }
}

fn parse_request_target(buf: &[u8]) -> Result<Vec<u8>, Utf8Error> {
    //the bytes that the buffer returns may or may not actually be text all the time
    //let actual_string = std::str::from_utf8(buf)?;
    //don't make a habit of converting everything to text
    //
    //
    //
    //
    //
    let request_str = str::from_utf8(buf)?;
    let index_of_start_req = str::find(request_str, "/");

    let mut request_line = vec![];

    if let Some(index) = index_of_start_req {
        for item in index..request_str.len() {
            if request_str.as_bytes()[item] == b' ' {
                //println!("we are indeed breaking");
                break;
            } else {
                request_line.push(request_str.as_bytes()[item]);
                continue;
            }
        }
    }

    //convert bytes of u8 vector into a string for display
    let request_line_str = str::from_utf8(&request_line);

    //print request line as a string
    request_line_str.iter().for_each(|element| {
        // println!("printing inside of loop");
        // println!("{:?}", element);
    });

    //now convert the buffer into ut8f text
    //at first just take a view into the u8 vec and borrow it using a byte slice
    //this avoids having to allocate a string or convert way too early and pass it around

    Ok(request_line.clone())
}
fn parse_content_len_and_string(target: &[u8]) -> Result<(usize, &str), ClientError> {
    let first_space = target.iter().position(|&b| b == b' ');
    let first_slash = target.iter().position(|&b| b == b'/');
    /*
     *
     *  use let else instead of verbose match here
     *   let Some(value) = optional_val else {
     *        return; // Failure case (must diverge)
     *  };
     *
     */

    let Some(space_idx) = first_space else {
        return Err(ClientError::RequestLineError(
            RequestLineError::SpaceParsing,
        ));
    };

    let Some(slash_idx) = first_slash else {
        return Err(ClientError::RequestLineError(
            RequestLineError::SlashParsing,
        ));
    };

    let second_space = target.iter().skip(space_idx).position(|&b| b == b' ');

    let Some(space_idx2) = second_space else {
        return Err(ClientError::RequestLineError(
            RequestLineError::SpaceParsing,
        ));
    };

    let second_slash = target.iter().skip(slash_idx).position(|&b| b == b'/');

    let Some(slash_idx2) = second_slash else {
        return Err(ClientError::RequestLineError(
            RequestLineError::SlashParsing,
        ));
    };

    if slash_idx2 > space_idx2 {
        return Err(ClientError::RequestLineError(
            RequestLineError::InvalidRange,
        ));
    }
    let byte_slice = &target[slash_idx2..space_idx2];

    Ok((byte_slice.len(), str::from_utf8(byte_slice)?))
}
fn parse_content_string() {
    todo!();
}
fn handle_client(stream: TcpStream, _listener: &TcpListener) -> Result<(), ClientError> {
    let mut owned_stream = stream;
    let mut buf: Vec<u8> = vec![0u8; 16834];

    let amt_bytes = owned_stream.read(&mut buf)?;

    debug_assert!(amt_bytes <= 16834);

    if amt_bytes > 16834 {
        let too_many_bytes_read = BufferError::TooManyBytesRead;
        return Err(ClientError::TcpStreamRead(too_many_bytes_read));
    }

    if is_get_request(&buf[..amt_bytes])? {
        let target = parse_request_target(&buf)?;

        let response = "HTTP/1.1 200 OK\r\n\r\n";
        let response404 = "HTTP/1.1 404 Not Found\r\n\r\n";
        let (len, str) = parse_content_len_and_string(&target)?;

        let response_echo = format!(
            "HTTP/1.1 200 Ok\r\nContent-Type: text/plain\r\nContent-Length: {:?}\r\n\r{:?}",
            len, str
        );

        if str::from_utf8(&target.clone())? == "/" {
            owned_stream.write_all(response.as_bytes())?;
        } else if str::from_utf8(&target)?.starts_with("/echo/") {
            //good example of what you want to return here
            //HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 3\r\n\r\nabc
            //
            let _response = owned_stream.write_all(&response_echo.into_bytes());

            //gotta parse the content and include it as the body though
        } else {
            owned_stream.write_all(response404.as_bytes())?;
        }
    } else {
        println!("033[31;43mWarning!\033[0m")
    }
    //implement from trait for Utf8Error to ClientError
    //you cannot use ? because it implicitly tries to cast utf8 error to client error

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

pub mod tests {
    use crate::parse_request_target;

    pub struct TestingBuf {
        request_line: &'static str,
    }
    #[test]
    pub fn return_request_line() -> Result<(), std::str::Utf8Error> {
        let test_struct = crate::tests::TestingBuf {
            request_line: "GET /index.html HTTP/1.1\r\nHost: localhost:4221\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n",
        };

        let target = parse_request_target(test_struct.request_line.as_bytes())?;

        assert_eq!(std::str::from_utf8(&target), Ok("/index.html"));

        Ok(())
    }
}

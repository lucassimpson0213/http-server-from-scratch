use crate::TcpListener;
use crate::errors::BufferError;
use crate::errors::ClientError;
use crate::request::is_get_request;
use crate::request::parse_content_len_and_string;
use crate::request::parse_headers;
use crate::request::parse_request_target;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

mod handler {
    use std::net::TcpStream;
    struct Request {
        amtbytes: usize,
        buffer: Vec<u8>,
    }

    impl Request {
        pub fn from_stream(stream: &TcpStream) {}
    }

    struct Response {}
}
pub fn handle_client(stream: TcpStream, _listener: &TcpListener) -> Result<(), ClientError> {
    let mut owned_stream = stream;
    let mut buf: Vec<u8> = Vec::new();

    let amt_bytes = owned_stream.read_to_end(&mut buf)?;

    debug_assert!(amt_bytes <= 16834);

    if amt_bytes > 16834 {
        let too_many_bytes_read = BufferError::TooManyBytesRead;
        return Err(ClientError::TcpStreamRead(too_many_bytes_read));
    }

    if !is_get_request(&buf)? {
        return Err(ClientError::UnsupportedRequest);
    }

    let target = parse_request_target(&buf)?;
    let (len, str) = parse_content_len_and_string(&target)?;
    let (len, header) = parse_headers(&buf);

    let response200 = "HTTP/1.1 200 OK\r\n\r\n";
    let response404 = "HTTP/1.1 404 Not Found\r\n\r\n";
    let response_echo = format!(
        "HTTP/1.1 200 Ok\r\nContent-Type: text/plain\r\nContent-Length: {:?}\r\n\r{:?}",
        len, str
    );

    if str::from_utf8(&target.clone())? == "/" {
        owned_stream.write_all(response200.as_bytes())?;
    } else if str::from_utf8(&target)?.starts_with("/echo/") {
        let _response = owned_stream.write_all(&response_echo.into_bytes());
    } else if str::from_utf8(&target)?.starts_with("/user-agent") {
    } else {
        owned_stream.write_all(response404.as_bytes())?;
    }

    Ok(())
}

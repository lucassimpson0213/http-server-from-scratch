#![deny(clippy::all)]
#![deny(clippy::unwrap_used)]
// #![deny(clippy::expect_used)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]
#![deny(clippy::dbg_macro)]
#![deny(unsafe_code)]
use crate::handler::handle_client;
use core::str::Utf8Error;
use std::net::TcpListener;
mod errors;
mod handler;
mod request;
mod response;

//TODO make a test script using curl localhost:4221/abcsad
fn main() {
    //println!("hello");

    let _ = bind_port();
}

fn bind_port() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:4221")?;

    for stream in listener.incoming() {
        let _ = handle_client(stream?, &listener);
    }

    Ok(())
}

#[cfg(test)]
pub mod tests {
    use crate::errors::ClientError;
    use crate::request::parse_content_len_and_string;
    use crate::request::parse_request_target;

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

    #[test]
    pub fn parse_content_unit_test() {
        let path_vec = [
            ("/echo/abc", Some((3_usize, "abc"))),
            ("/echo/", Some((0_usize, ""))),
            ("/echo/hello-world", Some((11_usize, "hello-world"))),
            ("/echo/with spaces", Some((11_usize, "with spaces"))),
            ("/echo/12345", Some((5_usize, "12345"))),
            ("/echo/a", Some((1_usize, "a"))),
            ("/echo/!", Some((1_usize, "!"))),
            ("/echo/hello_world", Some((11_usize, "hello_world"))),
            ("/echo/hello%20world", Some((13_usize, "hello%20world"))),
            ("/echo/abc/def", Some((7_usize, "abc/def"))),
            ("/echo//", Some((1_usize, "/"))),
            ("/echo/🦀", Some((4_usize, "🦀"))),
            ("/", None),
            ("/index.html", None),
            ("/echo", None),
            ("/echoe/abc", None),
            ("/Echo/abc", None),
            ("/api/echo/abc", None),
            ("echo/abc", None),
            ("", None),
        ];

        for (path, expected) in path_vec {
            let result = parse_content_len_and_string(path.as_bytes()).ok();

            assert_eq!(Some(result), Some(expected));
        }
    }
}

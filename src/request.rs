use crate::errors::ClientError;
use crate::errors::RequestLineError;
use std::str::Utf8Error;
pub fn is_get_request(buf: &[u8]) -> Result<bool, Utf8Error> {
    let request_str = str::from_utf8(buf)?;

    if request_str.contains("GET") {
        Ok(true)
    } else {
        Ok(false)
    }
}
pub fn parse_headers(request_line: &[u8]) -> (usize, &str) {
    println!("{:?}", request_line);
    (1_usize, "hello")
}
pub fn parse_request_target(buf: &[u8]) -> Result<Vec<u8>, Utf8Error> {
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

    Ok(request_line.clone())
}

pub fn parse_content_len_and_string(target: &[u8]) -> Result<(usize, &str), ClientError> {
    let slash = target.iter().position(|&b| b == b'/');

    let Some(slash_idx) = slash else {
        return Err(ClientError::RequestStringError(
            RequestLineError::SlashParsing,
        ));
    };
    let second_slash = target[slash_idx + 1..].iter().position(|&b| b == b'/');

    let Some(second_slash_idx) = second_slash else {
        return Err(ClientError::RequestStringError(
            RequestLineError::SlashParsing,
        ));
    };
    println!("{}", second_slash_idx);
    let byte_slice = &target[second_slash_idx + slash_idx + 2..];
    Ok((byte_slice.len(), str::from_utf8(byte_slice)?))
}

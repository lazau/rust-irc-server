use std::io;
use std::str;

use bytes::BytesMut;

use tokio_io::codec::{Encoder, Decoder};

#[derive(Debug)]
pub struct Utf8CrlfCodec;

impl Encoder for Utf8CrlfCodec {
    type Item = Vec<String>;
    type Error = io::Error;
    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        for message in item.iter() {
            // TODO(lazau): Don't unwrap.
            dst.extend(message.as_bytes());
            dst.extend(b"\r\n");
        }
        Ok(())
    }
}

impl Decoder for Utf8CrlfCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<String>, io::Error> {
        let mut crlf_pos: Option<usize> = None;
        for (pos, &c) in src.iter().enumerate() {
            if pos > 1 && c == b'\n' && src[pos - 1] == b'\r' {
                crlf_pos = Some(pos);
                break;
            }
        }

        match crlf_pos {
            Some(pos) => {
                let line = &src.split_to(pos + 1)[0..(pos - 1)];
                match str::from_utf8(&line) {
                    Ok(s) => Ok(Some(s.to_string())),
                    // TODO(lazau): Maybe optionally support ISO-8859-1?
                    Err(ref e) => {
                        debug!("Error: {:?}.", e.to_string());
                        Err(io::Error::new(
                            io::ErrorKind::Other,
                            "not valid utf-8 string",
                        ))
                    }
                }
            }
            None => Ok(None),
        }
    }

    // TODO(lazau): Maybe don't need to propagate EOF inband?
    fn decode_eof(&mut self, src: &mut BytesMut) -> Result<Option<String>, io::Error> {
        match try!(self.decode(src)) {
            Some(frame) => Ok(Some(frame)),
            None => Err(io::Error::new(io::ErrorKind::Other, "EOF")),
        }
    }
}

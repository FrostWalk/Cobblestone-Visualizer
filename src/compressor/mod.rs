use message_io::network::{Endpoint, NetworkController, SendStatus};
use serde::{Deserialize, Serialize};
use zstd::{decode_all, encode_all};

pub trait Compressor {
    fn send_compressed<T: ?Sized>(&self, endpoint: Endpoint, data: &T) -> Result<(), String>
        where T: serde::Serialize;
}

pub trait Expander<'b> {
    fn decode_decompress<'a, T>(&'b self) -> Result<T, String>
        where T: Deserialize<'b>;
}

const THRESHOLD_SIZE: usize = 200;
const COMPRESSION_LEVEL: i32 = 16;

#[derive(Serialize, Deserialize, Clone)]
struct Payload<'b> {
    data: &'b [u8],
    compressed: bool,
}

impl Compressor for NetworkController {
    fn send_compressed<T: ?Sized>(&self, endpoint: Endpoint, data: &T) -> Result<(), String>
        where T: serde::Serialize {
        let mut encoded = bincode::serialize(&data).map_err(|e| format!("{}", e))?;

        let compressed = encoded.len() >= THRESHOLD_SIZE;
        if compressed {
            encoded = encode_all(encoded.as_slice(), COMPRESSION_LEVEL).map_err(|e| format!("{}", e))?
        }

        let payload = bincode::serialize(&Payload { data: encoded, compressed }).map_err(|e| format!("{}", e))?;

        match self.send(endpoint, &payload) {
            SendStatus::Sent => { Ok(()) }
            SendStatus::MaxPacketSizeExceeded => {
                Err(format!("Max packet size exceed, size: {}byte", payload.len()))
            }
            SendStatus::ResourceNotFound => {
                Err(String::from("Connection was closed before sending"))
            }
            SendStatus::ResourceNotAvailable => {
                Err(String::from("Connection not ready, retry"))
            }
        }
    }
}

impl<'b> Expander <'b>for [u8] {
    fn decode_decompress<'a, T>(&'b self) -> Result<T, String>
        where T: Deserialize<'b> {
        let payload: Payload = bincode::deserialize(self).map_err(|e| format!("{}", e))?;
        let t = payload.data.as_slice();
        let decompressed = decode_all(t).map_err(|e| format!("{}", e))?.as_slice();
        if payload.compressed {
            bincode::deserialize::<'a, T>(decompressed).map_err(|e| format!("{}", e))
        } else {
            bincode::deserialize::<'a, T>(t).map_err(|e| format!("{}", e))
        }
    }
}

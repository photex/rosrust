use byteorder::{LittleEndian, WriteBytesExt};
use rustc_serialize::{Decodable, Encodable};
use std::collections::HashMap;
use std;
use super::{Decoder, Encoder};
use super::error::Error;

pub fn decode(data: Vec<u8>) -> Result<HashMap<String, String>, Error> {
    let vector_length = data.len();
    let reader = std::io::Cursor::new(data);
    let mut decoder = Decoder::new(reader);
    let length = decoder.pop_length()? as usize;
    if length + 4 != vector_length {
        return Err(Error::Mismatch);
    }
    let mut result = HashMap::<String, String>::new();
    let mut size_count = 0;
    while length > size_count {
        let point = String::decode(&mut decoder)?;
        size_count += point.len() + 4;
        let mut point = point.splitn(2, '=');
        let key = point.next().ok_or(Error::UnsupportedData)?;
        let value = point.next().ok_or(Error::UnsupportedData)?;
        result.insert(key.to_owned(), value.to_owned());
    }
    Ok(result)
}

pub fn encode(data: HashMap<String, String>) -> Result<Vec<u8>, Error> {
    let mut encoder = Encoder::new();
    for (key, value) in data {
        [key, value].join("=").encode(&mut encoder)?;
    }
    let mut buffer = Vec::new();
    let mut data = encoder.extract_data();
    buffer.reserve(4 + data.len());
    buffer.write_u32::<LittleEndian>(data.len() as u32)?;
    buffer.append(&mut data);
    Ok(buffer)
}

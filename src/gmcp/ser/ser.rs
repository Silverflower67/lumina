use serde::Serializer;
use super::error::{Error, Result};

pub trait Data {
    
}

pub fn to_gmcp<T>(value: &T) -> Result<Vec<u8>> where T: Serialize {
    let data = serde_json::to_string(value)
        .map_err(|err| Error::JSON(err))?;
    let mut sent: Vec<u8> = vec![0xff,250,201];
    sent.extend(data.as_bytes());
    sent.extend([0xff,240]);
    Ok(sent)
}

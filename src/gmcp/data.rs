use super::error::{Error, Result};
use serde::{Serialize, Deserialize};

/// A GMCP data struct
#[derive(Serialize,Deserialize)]
pub struct DataStruct<T> {
    /// The inner data structure
    pub inner: T, 
    pub name: String,
}

impl<'a,T> DataStruct<T> where T: Serialize + Deserialize<'a> {
    /// Creates a new DataStruct: `T` is a [Serialize](serde::Serialize) implementation
    pub fn new(data: T, name: String) -> Self {
        DataStruct {
            inner: data,
            name: name,
        }
    }
    pub fn to_str(&self) -> Result<String> {
        let mut s = String::new();
        s += &self.name;
        s += serde_json::to_string(&self.inner)
            .map_err(|err| Error::JSON(err))?
            .as_str();
        Ok(s)
    }
    pub fn from_str(data: &'a String) -> Result<Self> {
        let index = data.find("{").ok_or(Error::ExpectedGMCP)?;
        let name = &data[..index - 1].trim().clone();
        let index2 = &data[index..]
            .find("}")
            .ok_or(Error::Message("Expected end".to_string()))?;
        let inner: T =
            serde_json::from_str(&data[index..*index2]).map_err(|err| Error::JSON(err))?;
        Ok(Self::new(inner,name.to_owned().to_string()))
    }
}

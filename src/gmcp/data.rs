use super::error::{Error, Result};
use serde::{Serialize, Deserialize};

/// A GMCP data struct
#[derive(Serialize,Deserialize)]
pub struct DataStruct<T> {
    /// The inner data structure
    pub inner: T,
    /// The module name for the GMCP Table
    pub module: String,
    /// The title of the GMCP Table
    pub name: String,
}

impl<'a,T> DataStruct<T> where T: Serialize + Deserialize<'a> {
    /// Creates a new DataStruct: `T` is a [Serialize](serde::Serialize) implementation
    pub fn new(data: T, module: String, name: String) -> Self {
        DataStruct {
            inner: data,
            module: module,
            name,
        }
    }
    pub fn to_str(&self) -> Result<String> {
        let mut s = String::new();
        s += &format!("{}.{} ", self.module, self.name);
        s += serde_json::to_string(&self.inner)
            .map_err(|err| Error::JSON(err))?
            .as_str();
        Ok(s)
    }
    pub fn from_str(data: &'a String) -> Result<Self> {
        let index = data.find("{").ok_or(Error::ExpectedGMCP)?;
        let namevec = &data[..index - 1].trim().split(".").collect::<Vec<_>>();
        let (module, name) = namevec.split_first().unwrap();
        let index2 = &data[index..]
            .find("}")
            .ok_or(Error::Message("Expected end".to_string()))?;
        let inner: T =
            serde_json::from_str(&data[index..*index2]).map_err(|err| Error::JSON(err))?;
        Ok(Self::new(inner, module.to_string(), name.join(".")))
    }
}

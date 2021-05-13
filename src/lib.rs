mod de;
mod error;
mod ser;

pub use de::{from_bytes};
pub use ser::{to_bytes};

#[cfg(test)]
mod tests;

use bitfield::bitfield;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt;

bitfield! {
    pub struct Flags(u16);
    impl Debug;
    u16;
    pub ack, _: 0;
    pub ok, _: 1;
    pub rej, _: 2;
    pub end, _: 3;
    pub ner, _: 4;
    pub der, _: 5;
    pub ser, _: 6;
    reserved, _: 7, 15;
}
pub struct XpProtocol<T: DeserializeOwned + Serialize> {
    pub topic_id: u16,
    pub flags: Flags,
    length: Option<u32>,
    pub data: T,
}

impl<T: DeserializeOwned + Serialize> XpProtocol<T> {
    pub fn new<'a, E>(topic_id: u16, flags: Flags, data: &'a [u8], de: fn(&'a [u8]) -> Result<T, E>) -> Result<Self, E>
        where
            E: serde::de::Error + serde::ser::Error
    {
        Ok(Self {
            topic_id,
            flags,
            length: Some(data.len() as u32),
            data: de(data)?,
        })
    }

    pub fn new_typed(topic_id: u16, flags: Flags, data: T) -> Self {
        Self {
            topic_id,
            flags,
            length: None,
            data
        }
    }
}

impl<T: DeserializeOwned + Serialize + fmt::Debug> fmt::Debug for XpProtocol<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("XpProtocol")
          .field("topic_id", &self.topic_id)
          .field("flags", &self.flags.0)
          .field("length", &self.length)
          .field("data", &self.data)
          .finish()
    }
}

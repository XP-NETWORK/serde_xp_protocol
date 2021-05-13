mod de;
mod error;
mod ser;

pub use de::{from_bytes};
pub use ser::{to_bytes};

#[cfg(test)]
mod tests;

use bitfield::bitfield;
use serde::{Deserialize, Serialize};
use std::fmt;

bitfield! {
    pub struct Flags(MSB0 [u8]);
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
pub struct XpProtocol<'a, T: Deserialize<'a> + Serialize> {
    pub topic_id: u16,
    pub flags: Flags<&'a [u8]>,
    length: Option<u32>,
    pub data: T,
}

impl<'a, T: Deserialize<'a> + Serialize> XpProtocol<'a, T> {
    pub fn new<E>(topic_id: u16, flags: Flags<&'a [u8]>, data: &'a [u8], de: fn(&'a [u8]) -> Result<T, E>) -> Result<Self, E>
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

    pub fn new_typed(topic_id: u16, flags: Flags<&'a [u8]>, data: T) -> Self {
        Self {
            topic_id,
            flags,
            length: None,
            data
        }
    }
}

impl<'a, T: Deserialize<'a> + Serialize + fmt::Debug> fmt::Debug for XpProtocol<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("XpProtocol")
          .field("topic_id", &self.topic_id)
          .field("flags", &self.flags.0)
          .field("length", &self.length)
          .field("data", &self.data)
          .finish()
    }
}

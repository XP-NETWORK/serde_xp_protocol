mod de;
mod error; // TODO
mod ser;

pub use de::{from_bytes};
pub use ser::{to_bytes};

#[cfg(test)]
mod tests;

use bitfield::bitfield;
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
pub struct XpProtocol<'a> {
    pub topic_id: u16,
    pub flags: Flags,
    length: u32,
    pub data: &'a [u8],
}

impl<'a> XpProtocol<'a> {
    pub fn new(topic_id: u16, flags: Flags, data: &'a [u8]) -> Self {
        Self {
            topic_id,
            flags,
            length: data.len() as u32,
            data: data,
        }
    }
}

impl fmt::Debug for XpProtocol<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("XpProtocol")
          .field("topic_id", &self.topic_id)
          .field("flags", &self.flags.0)
          .field("length", &self.length)
          .field("data", &"...".to_string())
          .finish()
    }
}

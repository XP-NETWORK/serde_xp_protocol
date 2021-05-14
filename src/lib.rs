mod de;
mod ser;

pub use de::{from_bytes};
pub use ser::{to_bytes};

#[cfg(test)]
mod tests;

use bitfield::bitfield;
use std::fmt;
use serde::{Serialize, Deserialize};

type FlagsRaw = [u8; 2];

bitfield! {
    /// Flags is a 16bit Vector where each bit is the flag
    #[derive(Serialize, Deserialize, Clone)]
    #[serde(into = "FlagsRaw")]
    #[serde(from = "FlagsRaw")]
    pub struct Flags(u16);
    u16;
    pub ack, set_ack: 0;
    pub ok, set_ok: 1;
    pub rej, set_rej: 2;
    pub end, set_end: 3;
    pub ner, set_ner: 4;
    pub der, set_der: 5;
    pub col, set_col: 6;
    pub snt, set_snt: 7;
    pub exe, set_exe: 8;
    pub inf, set_inf: 9;
    reserved, _: 9, 15;
}

impl fmt::Debug for Flags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Flags")
          .field("ack", &self.ack())
          .field("ok", &self.ok())
          .field("rej", &self.rej())
          .field("end", &self.end())
          .field("ner", &self.ner())
          .field("der", &self.der())
          .field("col", &self.col())
          .field("snt", &self.snt())
          .field("exe", &self.exe())
          .field("inf", &self.inf())
          .finish()
    }
}

impl From<FlagsRaw> for Flags {
    fn from(v: FlagsRaw) -> Flags {
        Flags(u16::from_be_bytes(v))
    }
}

impl From<Flags> for FlagsRaw {
    fn from(v: Flags) -> FlagsRaw {
        v.0.to_be_bytes()
    }
}

/// XpProtocol Proposal
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct XpProtocol<'a> {
    /// Topic Id
    pub topic_id: u16,
    /// Flags
    pub flags: Flags,
    length: u32,
    /// Intention Data
    pub data: &'a [u8],
}

impl<'a> XpProtocol<'a> {
    /// Create a new Packet
    /// length is automatically taken from data
    pub fn new(topic_id: u16, flags: Flags, data: &'a [u8]) -> Self {
        Self {
            topic_id,
            flags,
            length: data.len() as u32,
            data: data,
        }
    }
}

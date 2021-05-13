use crate::{
    XpProtocol,
};

pub fn to_bytes<'a>(v: &XpProtocol<'a>) -> Vec<u8> {
    let mut res = Vec::new();
    res.extend(&v.topic_id.to_be_bytes());
    res.extend(&v.flags.0.to_be_bytes());
    res.extend(&v.length.to_be_bytes());
    res.extend(v.data);

    res
}

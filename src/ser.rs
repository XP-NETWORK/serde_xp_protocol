use crate::{
    XpProtocol,
};
use serde::{Serialize, Deserialize, ser, de};

pub fn to_bytes<'a, T: Serialize + Deserialize<'a>, E>(v: &XpProtocol<'a, T>, serializer: fn(&T) -> Result<Vec<u8>, E>) -> Result<Vec<u8>, E>
where
    E: ser::Error + de::Error
{
    let data = serializer(&v.data)?;
    let l = v.length.unwrap_or(data.len() as u32);

    let mut res = Vec::new();
    res.extend(&v.topic_id.to_be_bytes());
    res.extend(v.flags.0);
    res.extend(&l.to_be_bytes());
    res.extend(data);
    Ok(res)
}

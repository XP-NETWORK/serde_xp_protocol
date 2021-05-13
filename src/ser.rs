use crate::{
    XpProtocol,
};
use serde::{Serialize, de::DeserializeOwned, ser, de};

pub fn to_bytes<T: Serialize + DeserializeOwned, E>(v: &XpProtocol<T>, serializer: fn(&T) -> Result<Vec<u8>, E>) -> Result<Vec<u8>, E>
where
    E: ser::Error + de::Error
{
    let data = serializer(&v.data)?;
    let l = v.length.unwrap_or(data.len() as u32);

    let mut res = Vec::new();
    res.extend(&v.topic_id.to_be_bytes());
    res.extend(&v.flags.0.to_be_bytes());
    res.extend(&l.to_be_bytes());
    res.extend(data);
    Ok(res)
}

use crate::*;

#[test]
fn test_serialize() {
    let test_data = "{\"name\": \"test\"}";
    println!("test data: {}\n", test_data);

    let data: serde_json::Value = serde_json::from_str(test_data).unwrap();

    let prot = XpProtocol::new_typed(0, Flags(&[0u8, 0]), data);

    println!("Protocol Data: {:?}\n", prot);
    println!("Bytes: {}", hex::encode(to_bytes(&prot, serde_json::to_vec).unwrap()));
}


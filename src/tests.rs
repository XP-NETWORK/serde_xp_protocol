use crate::*;

#[test]
fn test_ser() {
    let test_data = "{\"name\": \"test\"}";
    println!("test data: {}\n", test_data);

    let data: serde_json::Value = serde_json::from_str(test_data).unwrap();

    let prot = XpProtocol::new_typed(0, Flags(&[0u8, 0]), data);

    println!("Protocol Data: {:?}\n", prot);
    println!("Bytes: {}", hex::encode(to_bytes(&prot, serde_json::to_vec).unwrap()));
}

#[test]
fn test_de() {
    let test_bytes = hex::decode("000000000000000f7b226e616d65223a2274657374227d").unwrap();
    let prot: XpProtocol<serde_json::Value> = from_bytes(&test_bytes, serde_json::from_slice).unwrap();
    assert_eq!(prot.topic_id, 0);

    println!("Decoded: {:?}\n", prot);
}

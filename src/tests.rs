use crate::*;

#[test]
fn test_ser() {
    let test_data = "{\"name\": \"test\"}";
    println!("test data: {}\n", test_data);

    let data: serde_json::Value = serde_json::from_str(test_data).unwrap();
    let by = serde_json::to_vec(&data).unwrap();

    let prot = XpProtocol::new(0, Flags(0b0000000001111011), &by);

    println!("Protocol Data: {:?}\n", prot);
    println!("Bytes: {}", hex::encode(to_bytes(&prot).unwrap()));
}

#[test]
fn test_de() {
    //let test_bytes = hex::decode("000000000000000f7b226e616d65223a2274657374227d").unwrap();
    //let prot: XpProtocol = from_bytes(&test_bytes).unwrap();
    //assert_eq!(prot.topic_id, 0);

    //println!("Decoded: {:?}\n", prot);
}

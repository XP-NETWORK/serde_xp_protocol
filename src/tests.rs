use crate::*;

#[test]
fn test_flags() {
    // reserved_, ser, der, ner, end, rej, ok, ack (LE system)
    let flags = Flags(0b0000000001101101);
    assert_eq!(flags.ack(), true);
    assert_eq!(flags.ok(), false);
    assert_eq!(flags.rej(), true);
    assert_eq!(flags.end(), true);
    assert_eq!(flags.ner(), false);
    assert_eq!(flags.der(), true);
    assert_eq!(flags.ser(), true);
    assert_eq!(std::mem::size_of::<Flags>(), 2);
}

#[test]
fn test_ser() {
    let test_data = "{\"name\": \"test\"}";
    println!("test data: {}\n", test_data);

    let data: serde_json::Value = serde_json::from_str(test_data).unwrap();
    let by = serde_json::to_vec(&data).unwrap();

    let prot = XpProtocol::new(0, Flags(0b0000000001101101), &by);

    println!("Protocol Data: {:?}\n", prot);
    println!("Bytes: {}", hex::encode(to_bytes(&prot).unwrap()));
}

#[test]
fn test_de() {
    let test_bytes = hex::decode("0000006d0000000f000000000000000f7b226e616d65223a2274657374227d").unwrap();
    let prot: XpProtocol = from_bytes(&test_bytes).unwrap();
    assert_eq!(prot.topic_id, 0);
    assert_eq!(prot.flags.0, 0b0000000001101101);

    let data: serde_json::Value = serde_json::from_slice(prot.data).unwrap();
    assert_eq!(data["name"], "test");
    println!("Decoded: {:?}\n", data);
}

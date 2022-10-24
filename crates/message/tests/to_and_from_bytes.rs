mod common;

#[test]
fn to_and_from_bytes() {
    let original_message = message::Message::new_unchoke();
    let message_as_bytes = original_message.into_bytes();
    let message_from_bytes = message::Message::from_bytes(message_as_bytes).unwrap();

    assert_eq!(message::Message::new_unchoke(), message_from_bytes);
}

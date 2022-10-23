mod common;

#[async_std::test]
async fn decode_torrent() {
    let data = common::read_torrent().unwrap();
    torrent::Torrent::from_bytes(data).await.unwrap();
}

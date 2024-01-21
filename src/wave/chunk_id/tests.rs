use super::ChunkId;

const ASCII: ChunkId = ChunkId::new(*b"TEST");
const INVALID: ChunkId = ChunkId::new([255, 127, 0, 128]);

#[test]
fn chunk_id_ascii_debug_std() {
    assert_eq!(format!("{:?}", ASCII), format!("{:?}", "TEST"));
}

#[test]
fn chunk_id_ascii_debug_alt() {
    assert_eq!(format!("{:#?}", ASCII), format!("{:#?}", "TEST"));
}

#[test]
fn chunk_id_ascii_display_std() {
    assert_eq!(format!("{}", ASCII), format!("{}", "TEST"));
}

#[test]
fn chunk_id_ascii_display_alt() {
    assert_eq!(format!("{:#}", ASCII), format!("{:#}", "TEST"));
}

#[test]
fn chunk_id_invalid_debug_std() {
    assert_eq!(format!("{:?}", INVALID), format!("{:08X}", 0xFF7F0080u32));
}

#[test]
fn chunk_id_invalid_debug_alt() {
    assert_eq!(format!("{:#?}", INVALID), format!("{:#08X}", 0xFF7F0080u32));
}

#[test]
fn chunk_id_invalid_display_std() {
    assert_eq!(format!("{}", INVALID), format!("{:08X}", 0xFF7F0080u32));
}

#[test]
fn chunk_id_invalid_display_alt() {
    assert_eq!(format!("{:#}", INVALID), format!("{:#08X}", 0xFF7F0080u32));
}

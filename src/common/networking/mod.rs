pub const PAYLOAD_LIMIT: usize = 65527;

pub type Buffer = Vec<u8>;

pub fn buffer() -> Buffer {
    vec![0; PAYLOAD_LIMIT]
}
use bytes::Bytes;
use bytes::BytesMut;

// TODO: inefficient

pub fn bytes_extend_from_slice(bytes: &mut Bytes, slice: &[u8]) {
    if slice.is_empty() {
        // nop
    } else if bytes.is_empty() {
        *bytes = Bytes::copy_from_slice(slice);
    } else {
        let mut new = BytesMut::with_capacity(bytes.len() + slice.len());
        new.extend_from_slice(&bytes);
        new.extend_from_slice(slice);
        *bytes = new.freeze();
    }
}

pub fn bytes_extend(bytes: &mut Bytes, slice: Bytes) {
    if bytes.is_empty() {
        *bytes = slice;
    } else {
        bytes_extend_from_slice(bytes, &slice);
    }
}


pub fn bytes_debug_output(bytes: &Bytes) {
        // println!("{}",std::str::from_utf8_unchecked(bytes));
        let data = bytes.to_vec();
        println!("bytes message output:{:#04X?}", bytes);
}
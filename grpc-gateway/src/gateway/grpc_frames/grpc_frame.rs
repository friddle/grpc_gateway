use std::collections::VecDeque;
use std::pin::Pin;
use std::task::Poll;

use bytes::{Buf, BytesMut};
use bytes::Bytes;
use futures::stream::Stream;
use futures::stream::StreamExt;
use futures::task::Context;

use crate::{Error, Result};

pub const GRPC_HEADER_LEN: usize = 5;

pub fn parse_grpc_frame_header<B: Buf>(stream: &mut B) -> Result<Option<u32>> {
    if stream.remaining() < GRPC_HEADER_LEN {
        return Ok(None);
    }

    let compressed = match stream.get_u8() {
        0 => false,
        1 => true,
        _ => return Err(Error::Other("unknown compression flag")),
    };
    if compressed {
        return Err(Error::Other("compression is not implemented"));
    }
    Ok(Some(stream.get_u32()))
}

pub fn write_grpc_frame_cb<F, E>(stream: &mut Vec<u8>, estimate: u32, frame: F) -> Result<()>
    where
        F: FnOnce(&mut Vec<u8>) -> Result<()>,
{
    stream.reserve(estimate as usize + GRPC_HEADER_LEN);

    stream.push(0); // compressed flag
    let size_pos = stream.len();
    stream.extend_from_slice(&[0, 0, 0, 0]); // len
    let frame_start = stream.len();
    frame(stream)?;
    let frame_size = stream.len() - frame_start;
    assert!(frame_size <= u32::max_value() as usize);
    stream[size_pos..size_pos + 4].copy_from_slice(&(frame_size as u32).to_be_bytes());
    Ok(())
}

//肯定又要优化的啦。。。但是先跑通再说->不过frame可以到外面在进行
//估计假如能解析相应的proto协议的话。那就Bytes可以玩出新的花样出来。当然应该没那么难。
pub fn write_grpc_frame_bytes(bytes: &Bytes, estimate: u32) -> Bytes
{
    let mut stream: Vec<u8> = Vec::new();
    stream.reserve(estimate as usize + GRPC_HEADER_LEN);
    stream.push(0); // compressed flag
    let size_pos = stream.len();
    stream.extend_from_slice(&[0, 0, 0, 0]); // len
    let frame_start = stream.len();
    stream.extend_from_slice(bytes);
    let frame_size = stream.len() - frame_start;
    assert!(frame_size <= u32::max_value() as usize);
    stream[size_pos..size_pos + 4].copy_from_slice(&(frame_size as u32).to_be_bytes());
    return Bytes::from(stream);
}

pub fn write_grpc_frame_bytes_v2(frame: Bytes, estimate: u32) -> Bytes {
    let mut byte = BytesMut::with_capacity(estimate as usize + GRPC_HEADER_LEN);
    let size_pos = 1;
    //0->falg位置
    byte.extend_from_slice(&[0]);
    let frame_start = 4 + 1;
    //长度位置
    byte.extend_from_slice(&[0, 0, 0, 0]);
    byte.extend_from_slice(frame.to_vec().as_slice());
    let last_frame_size = estimate - frame_start;
    byte[size_pos..size_pos + 4].copy_from_slice(&(last_frame_size as u32).to_be_bytes());
    let byte = byte.split();
    let byte = byte.freeze();
    return byte;
}
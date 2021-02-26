use bytes::Buf;
use bytes::Bytes;
use httpbis::BufGetBytes;
use httpbis::BytesDeque;

use crate::Error;
use crate::gateway::grpc_frames::grpc_frame::{GRPC_HEADER_LEN, parse_grpc_frame_header};
use crate::Result;

#[derive(Default, Debug)]
pub struct GrpcFrameParser {
    /// Next frame length; stripped prefix from the queue
    next_frame_len: Option<u32>,
    /// Unparsed bytes
    buffer: BytesDeque,
}

impl GrpcFrameParser {
    /// Add `Bytes` object to the queue.
    pub fn enqueue(&mut self, bytes: Bytes) {
        self.buffer.extend(bytes);
    }

    /// Try populating `next_frame_len` field from the queue.
    fn fill_next_frame_len(&mut self) -> Result<Option<u32>> {
        if let None = self.next_frame_len {
            self.next_frame_len = parse_grpc_frame_header(&mut self.buffer)?;
        }
        Ok(self.next_frame_len)
    }

    /// Parse next frame if buffer has full frame.
    pub fn next_frame(&mut self) -> Result<Option<(Bytes, usize)>> {
        if let Some(len) = self.fill_next_frame_len()? {
            if self.buffer.remaining() >= len as usize {
                self.next_frame_len = None;
                return Ok(Some((
                    BufGetBytes::get_bytes(&mut self.buffer, len as usize),
                    len as usize + GRPC_HEADER_LEN,
                )));
            }
        }
        Ok(None)
    }

    /// Parse all frames from buffer.
    pub fn next_frames(&mut self) -> Result<(Vec<Bytes>, usize)> {
        let mut r = Vec::new();
        let mut consumed = 0;
        while let Some((frame, frame_consumed)) = self.next_frame()? {
            r.push(frame);
            consumed += frame_consumed;
        }
        Ok((r, consumed))
    }

    /// Buffered data is empty.
    pub fn is_empty(&self) -> bool {
        self.next_frame_len.is_none() && !self.buffer.has_remaining()
    }

    pub fn check_empty(&self) -> Result<()> {
        if !self.is_empty() {
            return Err(Error::Other("partial frame"));
        }
        Ok(())
    }
}


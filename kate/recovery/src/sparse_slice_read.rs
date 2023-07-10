use core::iter::FromIterator;
use std::{
	collections::VecDeque,
	io::{Read, Result},
};

/// It is a Codec Reader which allows decoding from non-sequential data.
pub struct SparseSliceRead<'a> {
	parts: VecDeque<&'a [u8]>,
}

impl<'a> FromIterator<&'a [u8]> for SparseSliceRead<'a> {
	fn from_iter<I: IntoIterator<Item = &'a [u8]>>(iter: I) -> Self {
		let parts = VecDeque::from_iter(iter);
		Self { parts }
	}
}

impl<'a> Read for SparseSliceRead<'a> {
	fn read(&mut self, mut buf: &mut [u8]) -> Result<usize> {
		let mut bytes = 0usize;

		loop {
			let buf_len = buf.len();
			if buf_len == 0 || self.parts.is_empty() {
				break;
			}

			if let Some(next_part) = self.parts.pop_front() {
				// Define max copied bytes and pending for next iteration.
				let copied_len = std::cmp::min(next_part.len(), buf_len);
				bytes += copied_len;

				// Copy data into `buf`.
				let (source, pending_next_part) = next_part.split_at(copied_len);
				let (dest, pending_buf) = buf.split_at_mut(copied_len);
				dest.copy_from_slice(source);

				// Advance output buffer.
				buf = pending_buf;

				// Reinsert if it is still pending
				if !pending_next_part.is_empty() {
					self.parts.push_front(pending_next_part);
				}
			}
		}

		Ok(bytes)
	}
}

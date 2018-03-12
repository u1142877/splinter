/* Copyright (c) 2018 University of Utah
 *
 * Permission to use, copy, modify, and distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR(S) DISCLAIM ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL AUTHORS BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */

use bytes::{BigEndian, BufMut, Bytes, BytesMut, LittleEndian};

/// This type represents a read-only buffer of bytes that can be received from
/// the database. This type is primarily used to read objects from the database.
pub struct ReadBuf {
    // The inner `Bytes` that actually holds the data.
    inner: Bytes,
}

// Methods on ReadBuf.
impl ReadBuf {
    /// This method returns a ReadBuf. The returned type is basically a wrapper
    /// over a `Bytes` type.
    ///
    /// This function is marked `unsafe` to prevent extensions from constructing
    /// a `ReadBuf` on their own. The only way an extension should be able to
    /// see a `ReadBuf` is by making a get() call on some type that implements
    /// the `DB` trait.
    ///
    /// # Arguments
    ///
    /// * `buffer`: The underlying buffer that will be wrapped up inside a
    ///             `ReadBuf`.
    ///
    /// # Return
    /// The `ReadBuf` wrapping the passed in buffer.
    pub unsafe fn new(buffer: Bytes) -> ReadBuf {
        ReadBuf {
            inner: buffer,
        }
    }

    /// This method returns the number of bytes present inside the `ReadBuf`.
    ///
    /// # Return
    ///
    /// The number of bytes present inside the `ReadBuf`.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// This method indicates if the `ReadBuf` is empty.
    ///
    /// # Return
    ///
    /// True if the `ReadBuf` is empty. False otherwise.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// This method returns a slice of bytes to the data contained inside the
    /// `ReadBuf`.
    ///
    /// # Return
    ///
    /// A slice to the data contained inside the `ReadBuf`.
    pub fn read(&self) -> &[u8] {
        self.inner.as_ref()
    }
}

/// This type represents a read-write buffer of bytes that can be received from
/// the database. This type is primarily intended to be used to receive
/// allocations from, and write to the database.
pub struct WriteBuf {
    // Identifier for the data table this buffer will eventually be written to.
    table: u64,

    // The inner BytesMut that will actually be written to.
    inner: BytesMut,

    // The number of metadata bytes that was added in by the database when the
    // allocation was performed.
    meta_len: usize,
}

// Methods on WriteBuf.
impl WriteBuf {
    /// This method returns a `WriteBuf`. This returned type is basically a
    /// wrapper around a passed in buffer of type `BytesMut`.
    ///
    /// This method is marked unsafe to prevent extensions from constructing
    /// a `WriteBuf` of their own. The only way an extension should be able
    /// to see a `WriteBuf` is by making an alloc() call down to the database.
    ///
    /// # Arguments
    ///
    /// * `table`:  Identifier for the table the allocation was made for.
    /// * `buffer`: The underlying buffer that will be wrapped up inside a
    ///             `WriteBuf`.
    ///
    /// # Return
    /// The `WriteBuf` wrapping the passed in buffer.
    pub unsafe fn new(table: u64, buffer: BytesMut) -> WriteBuf {
        let init_len = buffer.len();

        WriteBuf {
            table: table,
            inner: buffer,
            meta_len: init_len,
        }
    }

    /// This method returns the number of bytes that have been written to the
    /// `WriteBuf` by the extension so far.
    ///
    /// # Return
    /// The number of bytes written to the `WriteBuf` by the extension so far.
    pub fn len(&self) -> usize {
        self.inner.len() - self.meta_len
    }

    /// This method returns the total number of bytes that can be written by an
    /// extension into the `WriteBuf`.
    ///
    /// # Return
    /// The total number of bytes that can be written into the `WriteBuf` by an
    /// extension.
    pub fn capacity(&self) -> usize {
        self.inner.capacity() - self.meta_len
    }

    /// This method writes a slice of bytes to the end of the `WriteBuf`.
    ///
    /// # Arguments
    ///
    /// * `data`: The slice of bytes to be written into the `WriteBuf`.
    ///
    /// # Abort
    ///
    /// This method will cause the extension to abort if there is insufficent
    /// space left inside the `WriteBuf` to perform the write.
    pub fn write_slice(&mut self, data: &[u8]) {
        self.inner.put_slice(data);
    }

    /// This method writes a single byte to the end of the `WriteBuf`.
    ///
    /// # Arguments
    ///
    /// * `data`: The byte to be written into the `WriteBuf`.
    ///
    /// # Abort
    ///
    /// This method will cause the extension to abort if there is insufficent
    /// space left inside the `WriteBuf` to perform the write.
    pub fn write_u8(&mut self, data: u8) {
        self.inner.put_u8(data);
    }

    /// This method writes a single u16 to the end of the `WriteBuf`.
    ///
    /// # Arguments
    ///
    /// * `data`: The u16 to be written into the `WriteBuf`.
    /// * `le`:   The ordering to be used while performing the write. If true,
    ///           little-endian will be used. If false, big-endian will be used.
    ///
    /// # Abort
    ///
    /// This method will cause the extension to abort if there is insufficent
    /// space left inside the `WriteBuf` to perform the write.
    pub fn write_u16(&mut self, data: u16, le: bool) {
        match le {
            true => { self.inner.put_u16::<LittleEndian>(data); }

            false => { self.inner.put_u16::<BigEndian>(data); }
        }
    }

    /// This method writes a single u32 to the end of the `WriteBuf`.
    ///
    /// # Arguments
    ///
    /// * `data`: The u32 to be written into the `WriteBuf`.
    /// * `le`:   The ordering to be used while performing the write. If true,
    ///           little-endian will be used. If false, big-endian will be used.
    ///
    /// # Abort
    ///
    /// This method will cause the extension to abort if there is insufficent
    /// space left inside the `WriteBuf` to perform the write.
    pub fn write_u32(&mut self, data: u32, le: bool) {
        match le {
            true => { self.inner.put_u32::<LittleEndian>(data); }

            false => { self.inner.put_u32::<BigEndian>(data); }
        }
    }

    /// This method writes a single u64 to the end of the `WriteBuf`.
    ///
    /// # Arguments
    ///
    /// * `data`: The u64 to be written into the `WriteBuf`.
    /// * `le`:   The ordering to be used while performing the write. If true,
    ///           little-endian will be used. If false, big-endian will be used.
    ///
    /// # Abort
    ///
    /// This method will cause the extension to abort if there is insufficent
    /// space left inside the `WriteBuf` to perform the write.
    pub fn write_u64(&mut self, data: u64, le: bool) {
        match le {
            true => { self.inner.put_u64::<LittleEndian>(data); }

            false => { self.inner.put_u64::<BigEndian>(data); }
        }
    }

    /// This method consumes the `WriteBuf`, returning a read-only view to the
    /// contained data.
    ///
    /// This method is marked unsafe to prevent extensions from calling it.
    ///
    /// # Return
    /// A `Bytes` handle to the underlying data that can no longer be mutated.
    pub unsafe fn freeze(self) -> (u64, Bytes) {
        (self.table, self.inner.freeze())
    }
}

// This module implements simple unit tests for ReadBuf and WriteBuf.
#[cfg(test)]
mod tests {
    use super::{ReadBuf, WriteBuf};
    use bytes::{BufMut, Bytes, BytesMut};

    // This method tests the "len()" method on ReadBuf.
    #[test]
    fn test_readbuf_len() {
        // Write 3 bytes into a BytesMut.
        let mut buf = BytesMut::with_capacity(10);
        buf.put_slice(&[1, 2, 3]);

        // Wrap the BytesMut inside a ReadBuf, and verify it's length.
        unsafe {
            let buf = ReadBuf::new(buf.freeze());
            assert_eq!(3, buf.len());
        }
    }

    // This method tests that "is_empty()" returns true when the ReadBuf
    // is empty.
    #[test]
    fn test_readbuf_isempty_true() {
        // Wrap a Bytes inside a ReadBuf, and verify that it is empty.
        unsafe {
            let buf = ReadBuf::new(Bytes::with_capacity(10));
            assert!(buf.is_empty());
        }
    }

    // This method tests that "is_empty()" returns false when the ReadBuf
    // is not empty.
    #[test]
    fn test_readbuf_isempty_false() {
        // Write 3 bytes into a BytesMut.
        let mut buf = BytesMut::with_capacity(10);
        buf.put_slice(&[1, 2, 3]);

        // Wrap the BytesMut inside a ReadBuf, and verify that it isn't empty.
        unsafe {
            let buf = ReadBuf::new(buf.freeze());
            assert!(!buf.is_empty());
        }
    }

    // This method tests the functionality of the "read()" method on ReadBuf.
    #[test]
    fn test_readbuf_read() {
        // Write data into a BytesMut.
        let mut buf = BytesMut::with_capacity(10);
        let data = &[1, 2, 3, 4, 5, 6, 7, 18, 19];
        buf.put_slice(data);

        // Wrap the BytesMut inside a ReadBuf, and verify that the contents
        // of the slice returned by "read()" match what was written in above.
        unsafe {
            let buf = ReadBuf::new(buf.freeze());
            assert_eq!(data, buf.read());
        }
    }

    // This method tests the functionality of the "len()" method on WriteBuf.
    #[test]
    fn test_writebuf_len() {
        // Allocate and write into a BytesMut.
        let mut buf = BytesMut::with_capacity(100);
        buf.put_slice(&[1; 30]);

        // Wrap up the above BytesMut inside a WriteBuf, and write more data in.
        // Verify that the length reported by len() does not include the data
        // written above.
        unsafe {
            let mut buf = WriteBuf::new(buf);
            let data = &[1, 2, 3, 4];
            buf.inner.put_slice(data);
            assert_eq!(data.len(), buf.len());
        }
    }

    // This method tests the functionality of the "capacity()" method on
    // WriteBuf.
    #[test]
    fn test_writebuf_capacity() {
        // Allocate and write into a BytesMut.
        let mut buf = BytesMut::with_capacity(100);
        let meta = &[1; 30];
        buf.put_slice(meta);

        // Wrap up the above BytesMut inside a WriteBuf, and verify that the
        // WriteBuf's capacity does not include the data written above.
        unsafe {
            let buf = WriteBuf::new(buf);
            assert_eq!(100 - meta.len(), buf.capacity());
        }
    }

    // This method tests the functionality of the "write_slice()" method on
    // WriteBuf.
    #[test]
    fn test_writebuf_writeslice() {
        // Create a WriteBuf, write into it with write_slice(), and the verify
        // that it's contents match what's expected.
        unsafe {
            let mut buf = WriteBuf::new(BytesMut::with_capacity(10));
            let data = &[1, 2, 3, 4, 5];
            buf.write_slice(data);
            assert_eq!(data, &buf.inner[..]);
        }
    }

    // This method tests that the "write_slice()" method on WriteBuf panics in
    // the case of a write overflow.
    #[test]
    #[should_panic]
    fn test_writebuf_writeslice_overflow() {
        // Create a WriteBuf, and write one byte more than it's capacity.
        unsafe {
            let mut buf = WriteBuf::new(BytesMut::with_capacity(100));
            let data = &[1; 101];
            buf.write_slice(data);
        }
    }

    // This method tests the functionality of the "write_u8()" method on
    // WriteBuf.
    #[test]
    fn test_writebuf_writeu8() {
        // Create a WriteBuf, write into it with write_u8(), and the verify
        // that it's contents match what's expected.
        unsafe {
            let mut buf = WriteBuf::new(BytesMut::with_capacity(10));
            buf.write_u8(200);

            let expected = &[200];
            assert_eq!(expected, &buf.inner[..]);
        }
    }

    // This method tests that the "write_u8()" method on WriteBuf panics in
    // the case of a write overflow.
    #[test]
    #[should_panic]
    fn test_writebuf_writeu8_overflow() {
        // Create a WriteBuf, and write one byte more than it's capacity.
        unsafe {
            let mut buf = WriteBuf::new(BytesMut::with_capacity(100));
            let data = &[1; 100];
            buf.write_slice(data);

            buf.write_u8(200);
        }
    }

    // This method tests the functionality of the "write_u16()" method on
    // WriteBuf, when the write order is Little endian.
    #[test]
    fn test_writebuf_writeu16_le() {
        // Create a WriteBuf, write into it with write_u16(), and the verify
        // that it's contents match what's expected.
        unsafe {
            let mut buf = WriteBuf::new(BytesMut::with_capacity(10));
            buf.write_u16(258, true);

            let expected = &[2, 1];
            assert_eq!(expected, &buf.inner[..]);
        }
    }

    // This method tests the functionality of the "write_u16()" method on
    // WriteBuf, when the write order is Big endian.
    #[test]
    fn test_writebuf_writeu16_be() {
        // Create a WriteBuf, write into it with write_u16(), and the verify
        // that it's contents match what's expected.
        unsafe {
            let mut buf = WriteBuf::new(BytesMut::with_capacity(10));
            buf.write_u16(258, false);

            let expected = &[1, 2];
            assert_eq!(expected, &buf.inner[..]);
        }
    }

    // This method tests that the "write_u16()" method on WriteBuf panics in
    // the case of a write overflow.
    #[test]
    #[should_panic]
    fn test_writebuf_writeu16_overflow() {
        // Create a WriteBuf, and write two bytes more than it's capacity.
        unsafe {
            let mut buf = WriteBuf::new(BytesMut::with_capacity(100));
            let data = &[1; 100];
            buf.write_slice(data);

            buf.write_u16(258, true);
        }
    }

    // This method tests the functionality of the "write_u32()" method on
    // WriteBuf, when the write order is Little endian.
    #[test]
    fn test_writebuf_writeu32_le() {
        // Create a WriteBuf, write into it with write_u32(), and the verify
        // that it's contents match what's expected.
        unsafe {
            let mut buf = WriteBuf::new(BytesMut::with_capacity(10));
            buf.write_u32(84148994, true);

            let expected = &[2, 3, 4, 5];
            assert_eq!(expected, &buf.inner[..]);
        }
    }

    // This method tests the functionality of the "write_u32()" method on
    // WriteBuf, when the write order is Big endian.
    #[test]
    fn test_writebuf_writeu32_be() {
        // Create a WriteBuf, write into it with write_u32(), and the verify
        // that it's contents match what's expected.
        unsafe {
            let mut buf = WriteBuf::new(BytesMut::with_capacity(10));
            buf.write_u32(84148994, false);

            let expected = &[5, 4, 3, 2];
            assert_eq!(expected, &buf.inner[..]);
        }
    }

    // This method tests that the "write_u32()" method on WriteBuf panics in
    // the case of a write overflow.
    #[test]
    #[should_panic]
    fn test_writebuf_writeu32_overflow() {
        // Create a WriteBuf, and write four bytes more than it's capacity.
        unsafe {
            let mut buf = WriteBuf::new(BytesMut::with_capacity(100));
            let data = &[1; 100];
            buf.write_slice(data);

            buf.write_u32(84148994, true);
        }
    }

    // This method tests the functionality of the "write_u64()" method on
    // WriteBuf, when the write order is Little endian.
    #[test]
    fn test_writebuf_writeu64_le() {
        // Create a WriteBuf, write into it with write_u64(), and the verify
        // that it's contents match what's expected.
        unsafe {
            let mut buf = WriteBuf::new(BytesMut::with_capacity(10));
            buf.write_u64(8674083586, true);

            let expected = &[2, 3, 4, 5, 2, 0, 0, 0];
            assert_eq!(expected, &buf.inner[..]);
        }
    }

    // This method tests the functionality of the "write_u64()" method on
    // WriteBuf, when the write order is Big endian.
    #[test]
    fn test_writebuf_writeu64_be() {
        // Create a WriteBuf, write into it with write_u64(), and then verify
        // that it's contents match what's expected.
        unsafe {
            let mut buf = WriteBuf::new(BytesMut::with_capacity(10));
            buf.write_u64(8674083586, false);

            let expected = &[0, 0, 0, 2, 5, 4, 3, 2];
            assert_eq!(expected, &buf.inner[..]);
        }
    }

    // This method tests that the "write_u64()" method on WriteBuf panics in
    // the case of a write overflow.
    #[test]
    #[should_panic]
    fn test_writebuf_writeu64_overflow() {
        // Create a WriteBuf, and write eight bytes more than it's capacity.
        unsafe {
            let mut buf = WriteBuf::new(BytesMut::with_capacity(100));
            let data = &[1; 100];
            buf.write_slice(data);

            buf.write_u64(8674083586, true);
        }
    }
}
use leaflet_network_buffer::McBuf;

pub struct PacketReader {
    data: Vec<u8>
}

impl PacketReader {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn append(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }
}

impl PacketReader {
    fn try_read_varint(&mut self, start_index: usize) -> Option<(i32, usize)> {
        let mut i = 0i32;
        let mut index = 0;
        loop {
            if index > 5 || start_index + index >= self.data.len() {
                return None;
            }

            i |= ((self.data[start_index + index] & 0x7F) as i32) << index * 7;
            index += 1;

            if (self.data[start_index + index] & 0x80) == 0 {
                return Some((i, index));
            }
        }
    }

    pub fn read_packet(&mut self) -> Option<McBuf> {
        let (length, offset) = self.try_read_varint(0)?;
        let length = length as usize;

        if self.data.len() < offset + length {
            None
        } else {
            self.data.drain(..offset);
            let data = self.data.drain(..length);
            let buf = McBuf::from_bytes(data.as_slice());
            Some(buf)
        }
    }
}
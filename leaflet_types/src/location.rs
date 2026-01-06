use leaflet_network_buffer::{BufferResult, McBuf, NetworkType};
use leaflet_macros::NetworkType;
use crate::identifier::Identifier;

#[derive(Debug)]
pub struct Position {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, NetworkType)]
pub struct Location {
    pub world: Identifier,
    pub location: Position,
}

impl NetworkType for Position {
    fn read(buf: &mut McBuf) -> BufferResult<Self> {
        let packed = buf.read_ulong()?;
        Ok(Self {
            x: (packed >> 38) as i32,
            y: ((packed << 52) >> 52) as i32,
            z: ((packed << 26) >> 38) as i32
        })
    }

    fn write(&self, buf: &mut McBuf) {
        buf.write_ulong(
            (((self.x & 0x3FFFFFF) as u64) << 38)
                | (((self.z & 0x3FFFFFF) as u64) << 12)
                | (self.y & 0xFFF) as u64,
        )
    }
}

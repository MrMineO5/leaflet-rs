use crate::{BufferResult, McBuf, NetworkType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VarInt(i32);
impl From<i32> for VarInt {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
impl From<VarInt> for i32 {
    fn from(value: VarInt) -> Self {
        value.0
    }
}

impl NetworkType for VarInt {
    fn read(buf: &mut McBuf) -> BufferResult<Self> {
        let value = buf.read_var_int()?;
        Ok(Self(value))
    }

    fn write(&self, buf: &mut McBuf) {
        buf.write_var_int(self.0);
    }
}

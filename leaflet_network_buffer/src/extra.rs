use uuid::Uuid;
use crate::{BufferResult, McBuf, NetworkType};

impl NetworkType for String {
    fn read(buf: &mut McBuf) -> BufferResult<Self> {
        Ok(buf.read_string(32767)?)
    }

    fn write(&self, buf: &mut McBuf) {
        buf.write_string(self)
    }
}

impl NetworkType for Uuid {
    fn read(buf: &mut McBuf) -> BufferResult<Self> {
        Ok(Uuid::from_bytes(buf.read_array::<16>()?))
    }

    fn write(&self, buf: &mut McBuf) {
        buf.write_slice(self.as_bytes())
    }
}

impl <T> NetworkType for Vec<T> where T : NetworkType {
    fn read(buf: &mut McBuf) -> BufferResult<Self> {
        let length = buf.read_var_int()?;
        let mut vec = Vec::with_capacity(length as usize);
        for _ in 0..length {
            vec.push(buf.read_network_type()?);
        }
        Ok(vec)
    }

    fn write(&self, buf: &mut McBuf) {
        buf.write_var_int(self.len() as i32);
        for item in self {
            buf.write_network_type(item);
        }
    }
}

impl <T> NetworkType for Option<T> where T : NetworkType {
    fn read(buf: &mut McBuf) -> BufferResult<Self> {
        if buf.read_bool()? {
            Ok(Some(buf.read_network_type()?))
        } else {
            Ok(None)
        }
    }

    fn write(&self, buf: &mut McBuf) {
        buf.write_bool(self.is_some());
        if let Some(item) = self {
            buf.write_network_type(item);
        }
    }
}

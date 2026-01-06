pub mod extra;
pub mod varint;

pub trait NetworkSerializer<T> where T: Sized {
    fn read(buf: &mut McBuf) -> BufferResult<T>;
    fn write(buf: &mut McBuf, value: &T);
}

pub trait NetworkType where Self: Sized {
    fn read(buf: &mut McBuf) -> BufferResult<Self>;
    fn write(&self, buf: &mut McBuf);
}

#[derive(Debug)]
pub enum BufferError {
    BufferUnderflow,
    StringTooLong,
    StringError(std::string::FromUtf8Error),
    VarIntOverflow,

    Generic(&'static str)
}

pub type BufferResult<T> = Result<T, BufferError>;

#[derive(Debug)]
pub struct McBuf {
    data: Vec<u8>,
    read_index: usize,
    write_index: usize
}

// Basic methods
impl McBuf {
    pub fn new() -> Self {
        Self { data: Vec::new(), read_index: 0, write_index: 0 }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self { data: bytes.to_vec(), read_index: 0, write_index: bytes.len() }
    }
}

impl McBuf {
    pub fn write_u8(&mut self, value: u8) {
        self.data.push(value);
        self.write_index += 1;
    }
    pub fn write_slice(&mut self, value: &[u8]) {
        self.data.extend_from_slice(value);
        self.write_index += value.len();
    }

    pub fn read_u8(&mut self) -> BufferResult<u8> {
        if self.read_index >= self.data.len() {
            return Err(BufferError::BufferUnderflow);
        }
        let dat = self.data[self.read_index];
        self.read_index += 1;
        Ok(dat)
    }
    pub fn read_array<const N: usize>(&mut self) -> BufferResult<[u8; N]> {
        if self.read_index + N > self.data.len() {
            return Err(BufferError::BufferUnderflow);
        }

        let start = self.read_index;
        let end = start + N;
        self.read_index = end;

        Ok(self.data[start..end].try_into().unwrap())
    }

    pub fn read_dyn_array(&mut self, length: usize) -> BufferResult<Vec<u8>> {
        let start = self.read_index;
        let end = start + length;
        self.read_index = end;
        Ok(self.data[start..end].to_vec())
    }

    pub fn as_slice(&self) -> &[u8] {
        self.data.as_slice()
    }

    pub fn length(&self) -> usize {
        self.data.len()
    }
}

// Write methods
impl McBuf {
    pub fn write_bool(&mut self, value: bool) {
        self.write_u8(value as u8);
    }

    pub fn write_byte(&mut self, value: i8) {
        self.write_u8(value as u8);
    }

    pub fn write_ubyte(&mut self, value: u8) {
        self.write_u8(value);
    }

    pub fn write_short(&mut self, value: i16) {
        self.write_slice(&value.to_be_bytes())
    }

    pub fn write_ushort(&mut self, value: u16) {
        self.write_slice(&value.to_be_bytes())
    }

    pub fn write_int(&mut self, value: i32) {
        self.write_slice(&value.to_be_bytes())
    }

    pub fn write_long(&mut self, value: i64) {
        self.write_slice(&value.to_be_bytes())
    }

    pub fn write_ulong(&mut self, value: u64) {
        self.write_slice(&value.to_be_bytes())
    }

    pub fn write_float(&mut self, value: f32) {
        self.write_slice(&value.to_be_bytes())
    }

    pub fn write_double(&mut self, value: f64) {
        self.write_slice(&value.to_be_bytes())
    }

    pub fn write_string(&mut self, value: &str) {
        self.write_var_int(value.len() as i32);
        self.write_slice(value.as_bytes())
    }

    // TODO: Write TextComponent

    pub fn write_var_int(&mut self, value: i32) {
        self.data.reserve(5);

        let mut rem = value;
        while (rem & !0x7F) != 0 {
            self.write_u8((rem & 0x7F) as u8 | 0x80);
            rem >>= 7;
        }
        self.write_u8(rem as u8)
    }

    pub fn write_var_long(&mut self, value: i64) {
        self.data.reserve(10);

        let mut rem = value;
        while (rem & !0x7F) != 0 {
            self.write_u8((rem & 0x7F) as u8 | 0x80);
            rem >>= 7;
        }
        self.write_u8(rem as u8)
    }

    pub fn write_network_type(&mut self, value: &impl NetworkType) {
        value.write(self);
    }
}

// Read methods
impl McBuf {
    pub fn read_bool(&mut self) -> BufferResult<bool> {
        Ok(self.read_u8()? != 0)
    }

    pub fn read_byte(&mut self) -> BufferResult<i8> {
        Ok(self.read_u8()? as i8)
    }

    pub fn read_ubyte(&mut self) -> BufferResult<u8> {
        self.read_u8()
    }

    pub fn read_short(&mut self) -> BufferResult<i16> {
        Ok(i16::from_be_bytes(self.read_array::<2>()?))
    }

    pub fn read_ushort(&mut self) -> BufferResult<u16> {
        Ok(u16::from_be_bytes(self.read_array::<2>()?))
    }

    pub fn read_int(&mut self) -> BufferResult<i32> {
        Ok(i32::from_be_bytes(self.read_array::<4>()?))
    }

    pub fn read_long(&mut self) -> BufferResult<i64> {
        Ok(i64::from_be_bytes(self.read_array::<8>()?))
    }

    pub fn read_ulong(&mut self) -> BufferResult<u64> {
        Ok(u64::from_be_bytes(self.read_array::<8>()?))
    }

    pub fn read_float(&mut self) -> BufferResult<f32> {
        Ok(f32::from_be_bytes(self.read_array::<4>()?))
    }

    pub fn read_double(&mut self) -> BufferResult<f64> {
        Ok(f64::from_be_bytes(self.read_array::<8>()?))
    }

    pub fn read_string(&mut self, max_length: usize) -> BufferResult<String> {
        let length = self.read_var_int()? as usize;
        if length > max_length * 3 {
            return Err(BufferError::StringTooLong)
        }

        let string = String::from_utf8(self.read_dyn_array(length)?)
            .map_err(|e| BufferError::StringError(e))?;

        if string.len() > max_length {
            return Err(BufferError::StringTooLong)
        }

        Ok(string)
    }

    pub fn read_var_int(&mut self) -> BufferResult<i32> {
        let mut i = 0i32;
        let mut position = 0;

        loop {
            if position >= 35 {
                return Err(BufferError::VarIntOverflow);
            }

            let byte = self.read_u8()?;
            i |= ((byte & 0x7F) as i32) << position;
            position += 7;

            if (byte & 0x80) == 0 {
                break;
            }
        }

        Ok(i)
    }

    pub fn read_var_long(&mut self) -> BufferResult<i64> {
        let mut i = 0i64;
        let mut position = 0;

        loop {
            if position >= 70 {
                return Err(BufferError::VarIntOverflow);
            }

            let byte = self.read_u8()?;
            i |= ((byte & 0x7F) as i64) << position;
            position += 7;

            if (byte & 0x80) == 0 {
                break;
            }
        }

        Ok(i)
    }

    pub fn read_network_type<T: NetworkType>(&mut self) -> BufferResult<T> {
        T::read(self)
    }
}


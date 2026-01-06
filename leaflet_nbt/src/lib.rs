use leaflet_network_buffer::{BufferError, BufferResult, McBuf, NetworkType};

#[derive(Debug)]
pub enum TagContent {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(Vec<NBTTag>),
    Compound(Vec<NBTTag>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

#[derive(Debug)]
pub struct NBTTag {
    pub content: TagContent,
    pub name: Option<String>,
}

impl NBTTag {
    pub fn boolean(name: Option<String>, value: bool) -> Self {
        Self {
            content: TagContent::Byte(if value { 1 } else { 0 }),
            name: name.into(),
        }
    }
    
    pub fn byte(name: Option<String>, value: i8) -> Self {
        Self {
            content: TagContent::Byte(value),
            name,
        }
    }
    
    pub fn short(name: Option<String>, value: i16) -> Self {
        Self {
            content: TagContent::Short(value),
            name,
        }
    }
    
    pub fn int(name: Option<String>, value: i32) -> Self {
        Self {
            content: TagContent::Int(value),
            name,
        }
    }
    
    pub fn long(name: Option<String>, value: i64) -> Self {
        Self {
            content: TagContent::Long(value),
            name,
        }
    }
    
    pub fn float(name: Option<String>, value: f32) -> Self {
        Self {
            content: TagContent::Float(value),
            name,
        }
    }
    
    pub fn double(name: Option<String>, value: f64) -> Self {
        Self {
            content: TagContent::Double(value),
            name,
        }
    }
    
    pub fn byte_array(name: Option<String>, value: Vec<i8>) -> Self {
        Self {
            content: TagContent::ByteArray(value),
            name,
        }
    }
    
    pub fn string(name: Option<String>, value: String) -> Self {
        Self {
            content: TagContent::String(value),
            name,
        }
    }
    
    pub fn list(name: Option<String>, tags: Vec<NBTTag>) -> Self {
        // TODO: Check all tags have the same type
        Self {
            content: TagContent::List(tags),
            name,
        }
    }
    
    pub fn compound(name: Option<String>, tags: Vec<NBTTag>) -> Self {
        Self {
            content: TagContent::Compound(tags),
            name,
        }
    }
    
    pub fn int_array(name: Option<String>, values: Vec<i32>) -> Self {
        Self {
            content: TagContent::IntArray(values),
            name,
        }
    }
    
    pub fn long_array(name: Option<String>, values: Vec<i64>) -> Self {
        Self {
            content: TagContent::LongArray(values),
            name,
        }
    }
}

impl TagContent {
    pub fn type_id(&self) -> u8 {
        match self {
            TagContent::End => 0,
            TagContent::Byte(_) => 1,
            TagContent::Short(_) => 2,
            TagContent::Int(_) => 3,
            TagContent::Long(_) => 4,
            TagContent::Float(_) => 5,
            TagContent::Double(_) => 6,
            TagContent::ByteArray(_) => 7,
            TagContent::String(_) => 8,
            TagContent::List(_) => 9,
            TagContent::Compound(_) => 10,
            TagContent::IntArray(_) => 11,
            TagContent::LongArray(_) => 12,
        }
    }

    pub fn write_id(&self, buf: &mut McBuf) {
        buf.write_u8(self.type_id())
    }

    pub fn write_content(&self, buf: &mut McBuf) {
        match self {
            TagContent::End => {}
            TagContent::Byte(value) => buf.write_byte(*value),
            TagContent::Short(value) => buf.write_short(*value),
            TagContent::Int(value) => buf.write_int(*value),
            TagContent::Long(value) => buf.write_long(*value),
            TagContent::Float(value) => buf.write_float(*value),
            TagContent::Double(value) => buf.write_double(*value),
            TagContent::ByteArray(value) => {
                buf.write_int(value.len() as i32);
                for item in value {
                    buf.write_byte(*item);
                }
            }
            TagContent::String(value) => {
                let bytes = value.as_bytes();
                buf.write_ushort(bytes.len() as u16);
                buf.write_slice(bytes);
            }
            TagContent::List(value) => {
                if value.is_empty() {
                    buf.write_u8(0);
                    buf.write_int(0);
                    return;
                }

                let type_id = value[0].content.type_id();
                buf.write_u8(type_id);
                buf.write_int(value.len() as i32);
                for item in value {
                    if item.content.type_id() != type_id {
                        panic!("List item type mismatch")
                    }
                    item.write_name_and_content(buf);
                }
            }
            TagContent::Compound(value) => {
                for item in value {
                    item.content.write_id(buf);
                    item.write_name_and_content(buf);
                }
                buf.write_u8(0);
            }
            TagContent::IntArray(value) => {
                buf.write_int(value.len() as i32);
                for item in value {
                    buf.write_int(*item);
                }
            }
            TagContent::LongArray(value) => {
                buf.write_int(value.len() as i32);
                for item in value {
                    buf.write_long(*item);
                }
            }
        }
    }

    pub fn read_content(type_id: u8, buf: &mut McBuf) -> BufferResult<TagContent> {
        match type_id {
            0 => Ok(TagContent::End),
            1 => Ok(TagContent::Byte(buf.read_byte()?)),
            2 => Ok(TagContent::Short(buf.read_short()?)),
            3 => Ok(TagContent::Int(buf.read_int()?)),
            4 => Ok(TagContent::Long(buf.read_long()?)),
            5 => Ok(TagContent::Float(buf.read_float()?)),
            6 => Ok(TagContent::Double(buf.read_double()?)),
            7 => {
                let len = buf.read_int()? as usize;
                let mut result = Vec::with_capacity(len);
                for _ in 0..len {
                    result.push(buf.read_byte()?);
                }
                Ok(TagContent::ByteArray(result))
            }
            8 => {
                let len = buf.read_short()? as usize;
                Ok(TagContent::String(
                    String::from_utf8(buf.read_dyn_array(len)?)
                        .map_err(|e| BufferError::StringError(e))?,
                ))
            }
            9 => {
                let type_id = buf.read_u8()?;
                let len = buf.read_int()? as usize;
                let mut result = Vec::with_capacity(len);
                for _ in 0..len {
                    let name = NBTTag::read_name(buf)?;
                    let content = TagContent::read_content(type_id, buf)?;
                    result.push(NBTTag { content, name });
                }
                Ok(TagContent::List(result))
            }
            10 => {
                let mut result = Vec::new();
                loop {
                    let type_id = buf.read_u8()?;
                    if type_id == 0 {
                        break
                    }

                    let name = NBTTag::read_name(buf)?;
                    let content = TagContent::read_content(type_id, buf)?;
                    result.push(NBTTag { content, name });
                }
                Ok(TagContent::Compound(result))
            }
            11 => {
                let len = buf.read_int()? as usize;
                let mut result = Vec::with_capacity(len);
                for _ in 0..len {
                    result.push(buf.read_int()?);
                }
                Ok(TagContent::IntArray(result))
            }
            12 => {
                let len = buf.read_int()? as usize;
                let mut result = Vec::with_capacity(len);
                for _ in 0..len {
                    result.push(buf.read_long()?);
                }
                Ok(TagContent::LongArray(result))
            }
            _ => Err(BufferError::Generic("Unknown tag type")),
        }
    }
}

impl NBTTag {
    pub fn write_name(&self, buf: &mut McBuf) {
        match &self.name {
            Some(name) => {
                let bytes = name.as_bytes();
                buf.write_ushort(bytes.len() as u16);
                buf.write_slice(bytes);
            }
            None => buf.write_ushort(0),
        }
    }

    pub fn write_name_and_content(&self, buf: &mut McBuf) {
        self.write_name(buf);
        self.content.write_content(buf);
    }

    pub fn read_name(buf: &mut McBuf) -> BufferResult<Option<String>> {
        let len = buf.read_ushort()? as usize;
        if len == 0 {
            Ok(None)
        } else {
            Ok(Some(
                String::from_utf8(buf.read_dyn_array(len)?)
                    .map_err(|e| BufferError::StringError(e))?
            ))
        }
    }
}

impl NetworkType for NBTTag {
    fn read(buf: &mut McBuf) -> BufferResult<Self> {
        let type_id = buf.read_u8()?;
        let name = NBTTag::read_name(buf)?;
        let content = TagContent::read_content(type_id, buf)?;
        Ok(NBTTag {
            content,
            name,
        })
    }

    fn write(&self, buf: &mut McBuf) {
        self.content.write_id(buf);
        self.content.write_content(buf);
    }
}

use std::fmt::Display;
use leaflet_network_buffer::{BufferResult, McBuf, NetworkType};

#[derive(Debug, Clone)]
pub struct Identifier {
    namespace: String,
    value: String,
}

impl Identifier {
    pub fn new(namespace: &str, value: &str) -> Self {
        Self {
            namespace: namespace.to_string(),
            value: value.to_string(),
        }
    }

    pub fn minecraft(value: &str) -> Self {
        Self::new("minecraft", value)
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{}:{}", self.namespace, self.value))
    }
}

impl NetworkType for Identifier {
    fn read(buf: &mut McBuf) -> BufferResult<Self> {
        buf.read_string(32767)
            .map(|s| Self::new(&s[..s.find(':').unwrap()], &s[s.find(':').unwrap() + 1..]))
    }

    fn write(&self, buf: &mut McBuf) {
        buf.write_string(&format!("{}:{}", self.namespace, self.value))
    }
}

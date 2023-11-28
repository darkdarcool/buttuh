// const MAX_STRING_LENGTH: u16 = 32767;
/// A struct that allows for easy writing of bytes.
pub struct Writer {
    buf: Vec<u8>,
}

impl Writer {
    pub fn new() -> Self {
        Writer { buf: Vec::new() }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.buf
    }

    pub fn write_byte(&mut self, x: i8) -> &mut Self {
        self.buf.push(x as u8);
        self
    }

    pub fn write_unsigned_byte(&mut self, x: u8) -> &mut Self {
        self.buf.push(x);
        self
    }

    pub fn write_short(&mut self, x: i16) -> &mut Self {
        self.buf.extend_from_slice(&x.to_le_bytes());
        self
    }

    pub fn write_unsigned_short(&mut self, x: u16) -> &mut Self {
        let mut bytes = x.to_le_bytes();
        bytes.reverse(); // whyyyyyyyyy i hate rust
        self.buf.extend_from_slice(&bytes);
        self
    }

    pub fn write_int(&mut self, x: i32) -> &mut Self {
        self.buf.extend_from_slice(&x.to_le_bytes());
        self
    }

    pub fn write_unsigned_int(&mut self, x: u32) -> &mut Self {
        self.buf.extend_from_slice(&x.to_le_bytes());
        self
    }

    pub fn write_var_int(&mut self, mut value: i32) -> &mut Self {
        loop {
            let mut temp = (value & 0x7F) as u8;
            value >>= 7;
            if value != 0 {
                temp |= 0x80;
            }
            self.buf.push(temp);
            if value == 0 {
                break;
            }
        }
        self
    }

    pub fn write_long(&mut self, x: i64) -> &mut Self {
        self.buf.extend_from_slice(&x.to_le_bytes());
        self
    }

    pub fn write_string(&mut self, x: &str) -> &mut Self {
        let bytes = x.as_bytes();
        self.write_var_int(bytes.len() as i32);
        self.buf.extend_from_slice(bytes);
        self
    }
}

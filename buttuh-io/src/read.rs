/// Create a new reader from a buffer
pub struct Reader {
    buf: Vec<u8>,
    pos: usize,
}

impl Reader {
    pub fn new(buf: Vec<u8>) -> Self {
        Reader { buf, pos: 0 }
    }

    pub fn read_byte(&mut self) -> i8 {
        let byte = self.buf[self.pos] as i8;
        self.pos += 1;
        byte
    }

    pub fn read_unsigned_byte(&mut self) -> u8 {
        let byte = self.buf[self.pos];
        self.pos += 1;
        byte
    }

    pub fn read_short(&mut self) -> i16 {
        let mut bytes = [0; 2];
        bytes.copy_from_slice(&self.buf[self.pos..self.pos + 2]);
        self.pos += 2;
        i16::from_le_bytes(bytes)
    }

    pub fn read_unsigned_short(&mut self) -> u16 {
        let mut bytes = [0; 2];
        bytes.copy_from_slice(&self.buf[self.pos..self.pos + 2]);
        self.pos += 2;
        u16::from_le_bytes(bytes)
    }

    pub fn read_int(&mut self) -> i32 {
        let mut bytes = [0; 4];
        bytes.copy_from_slice(&self.buf[self.pos..self.pos + 4]);
        self.pos += 4;
        i32::from_le_bytes(bytes)
    }

    pub fn read_unsigned_int(&mut self) -> u32 {
        let mut bytes = [0; 4];
        bytes.copy_from_slice(&self.buf[self.pos..self.pos + 4]);
        self.pos += 4;
        u32::from_le_bytes(bytes)
    }

    pub fn read_var_int(&mut self) -> i32 {
        let mut num_read = 0;
        let mut result = 0;
        let mut read: i8;
        loop {
            read = self.read_byte();
            let value = (read & 0b01111111) as i32;
            result |= value << (7 * num_read);
            num_read += 1;
            if num_read > 5 {
                panic!("VarInt is too big");
            }
            if (read & 0b10000000u8 as i8) == 0 {
                break;
            }
        }
        result
    }

    pub fn read_string(&mut self) -> String {
        let length = self.read_var_int();
        let mut bytes = vec![0; length as usize];
        bytes.copy_from_slice(&self.buf[self.pos..self.pos + length as usize]);
        self.pos += length as usize;
        // remove first 3 bytes bc encoding sucks
        bytes = bytes[3..].to_vec();
        bytes.iter().map(|&c| c as char).collect::<String>()
    }

    pub fn read_long(&mut self) -> i64 {
        let mut bytes = [0; 8];
        bytes.copy_from_slice(&self.buf[self.pos..self.pos + 8]);
        self.pos += 8;
        i64::from_le_bytes(bytes)
    }

    pub fn read_unsigned_long(&mut self) -> u64 {
        let mut bytes = [0; 8];
        bytes.copy_from_slice(&self.buf[self.pos..self.pos + 8]);
        self.pos += 8;
        u64::from_le_bytes(bytes)
    }

    pub fn read_float(&mut self) -> f32 {
        let mut bytes = [0; 4];
        bytes.copy_from_slice(&self.buf[self.pos..self.pos + 4]);
        self.pos += 4;
        f32::from_le_bytes(bytes)
    }

    pub fn read_double(&mut self) -> f64 {
        let mut bytes = [0; 8];
        bytes.copy_from_slice(&self.buf[self.pos..self.pos + 8]);
        self.pos += 8;
        f64::from_le_bytes(bytes)
    }

    pub fn read_bytes(&mut self, length: usize) -> Vec<u8> {
        let mut bytes = vec![0; length];
        bytes.copy_from_slice(&self.buf[self.pos..self.pos + length]);
        self.pos += length;
        bytes
    }

    pub fn read_bool(&mut self) -> bool {
        self.read_byte() != 0
    }

    pub fn read_uuid(&mut self) -> String {
        let mut bytes = [0; 16];
        bytes.copy_from_slice(&self.buf[self.pos..self.pos + 16]);
        self.pos += 16;
        format!(
            "{:x}-{:x}-{:x}-{:x}-{:x}",
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4]
        )
    }
}

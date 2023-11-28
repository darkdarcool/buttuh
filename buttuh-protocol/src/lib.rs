use buttuh_io::write;

#[derive(Debug)]
/// Represents a serverbound handshake packet. Used to initiate a handshake with a server.
pub struct ServerboundHandshakePacket {
    /// Represents the MC client's protocol version.
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    /// 1 for status, 2 for login
    pub next_state: i32, 
}

impl ServerboundHandshakePacket {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut writer = write::Writer::new();
        writer.write_var_int(0x00);
        writer.write_var_int(self.protocol_version);
        writer.write_string(&self.server_address);
        writer.write_unsigned_short(self.server_port);
        writer.write_var_int(self.next_state);
        writer.bytes().to_vec()
    }
}

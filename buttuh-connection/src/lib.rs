use buttuh_io::utils;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
/// Represents a connection to a Minecraft server.
pub struct ButtuhServer {
    stream: TcpStream,
}

impl ButtuhServer {
    /// Creates a new connection to a Minecraft server.
    pub async fn new() -> Self {
        ButtuhServer {
            stream: TcpStream::connect("mc.advancius.net:25565").await.unwrap(),
        }
    }
    /// Sends a packet to the server.
    /// Note that after each packet is sent, the stream is flushed.
    pub async fn send_packet(&mut self, packet: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let packet_len = utils::get_packet_len(&packet);
        self.stream.write(&packet_len).await?; // send packet length first
        self.stream.write(&packet).await?; // packet id, and data
        self.stream.flush().await?; // flush the stream to ensure data is sent before proceeding
        Ok(())
    }
    
    /// Reads a packet from the server.
    pub async fn read_packet(&mut self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // use chunks instead of reading the whole thing at once
        let mut buf = Vec::new();
        let mut chunk = [0; 4096];
        loop {
            let bytes_read = self.stream.read(&mut chunk).await?;
            if bytes_read == 0 {
                break;
            }
            buf.extend_from_slice(&chunk[..bytes_read]);
        }

        Ok(buf)
    }
}

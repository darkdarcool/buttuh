/// Get the length of a packet
pub fn get_packet_len(packet: &Vec<u8>) -> [u8; 1] {
    let packet_len = packet.len();
    [(packet_len as u8)] // converting to bytes caused some problems
}

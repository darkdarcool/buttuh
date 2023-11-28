use buttuh_connection::ButtuhServer;
use buttuh_protocol::ServerboundHandshakePacket;

#[tokio::main]
async fn main() {
    let serverbound_handshake_packet = ServerboundHandshakePacket {
        protocol_version: 760,
        server_address: String::from("mc.advancius.net"),
        server_port: 25565,
        next_state: 1,
    };
    let mut connection = ButtuhServer::new().await;

    connection
        .send_packet(serverbound_handshake_packet.to_bytes())
        .await
        .unwrap();
    connection.send_packet(vec![0x00]).await.unwrap();

    let response = connection.read_packet().await.unwrap();

    let mut reader = buttuh_io::read::Reader::new(response);

    // let packet_len = reader.read_var_int();
    let packet_data = reader.read_string(); // this is a special packet with a json string
    println!("{}", packet_data);
}

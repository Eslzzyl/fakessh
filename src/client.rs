use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::net::TcpStream;
use crate::byte_reader::ByteReader;

const PROTO_VERSION_EXCHANGE: &str = "SSH-2.0-SSHServer\r\n";
// https://tools.ietf.org/html/rfc4253#section-6.1
const MAX_PACKET_SIZE: usize = 35000;

pub struct Client {
    socket: TcpStream,
    protocol_version_exchange: String,
}

impl Client {
    pub fn from(tcp_stream: TcpStream) -> Self {
        Self {
            socket: tcp_stream,
            protocol_version_exchange: String::new(),
        }
    }

    async fn send(&mut self, data: &[u8]) {
        self.socket.write_all(data).await.unwrap();
    }

    async fn read_protocol_version_exchange(&mut self) {
        let mut result = String::new();

        loop {
            let mut buffer = [0; 1];
            self.socket.read(&mut buffer).await.unwrap();
            let byte = buffer[0];

            if byte == b'\r' {
                // 再读入一个字节（应为\n）
                self.socket.read(&mut buffer).await.unwrap();
                break;
            }
            
            result.push(byte as char);
        }

        self.protocol_version_exchange = result;
    }

    async fn read_packet(&mut self) -> Option<Vec<u8>> {
        const BLOCK_SIZE: usize = 8;
        let mut first_block = vec![0u8; BLOCK_SIZE];
        let bytes_read = self.socket.read(&mut first_block).await.unwrap();
        if bytes_read != BLOCK_SIZE {
            return None;
        }
        let mut reader = ByteReader::from(first_block.clone());
        let packet_length = reader.get_uint32();
        if packet_length as usize > MAX_PACKET_SIZE {
            return None;
        }
        let padding_length = reader.get_byte();
        let bytes_to_read: usize = packet_length as usize - BLOCK_SIZE + 4;
        let mut rest_of_packet = vec![0u8; bytes_to_read];
        let bytes_read = self.socket.read(&mut rest_of_packet).await.unwrap();
        if bytes_read != bytes_to_read {
            return None;
        }
        let payload_length = packet_length as usize - padding_length as usize - 1;
        let full_packet = [first_block, rest_of_packet].concat();
        // 截取payload
        let payload = full_packet[5..payload_length].to_vec();

        Some(payload)
    }

    pub async fn process(&mut self) {
        self.send(&PROTO_VERSION_EXCHANGE.as_bytes()).await;
        self.read_protocol_version_exchange().await;
        println!("{:?}", self.protocol_version_exchange);
        loop {
            let packet = self.read_packet().await;
            if packet.is_none() {
                break;
            }
            println!("{:?}", packet.unwrap());
        }
    } 
}

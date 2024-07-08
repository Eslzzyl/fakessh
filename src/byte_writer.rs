use std::io::{Cursor, Write};
use crate::constant::PacketType;

pub struct ByteWriter {
    data: Cursor<Vec<u8>>
}

impl ByteWriter {
    pub fn new() -> Self {
        Self {
            data: Cursor::new(Vec::new())
        }
    }
    
    pub fn get_bytes(&self) -> Vec<u8> {
        self.data.get_ref().clone()
    }

    pub fn write_packet_type(&mut self, packet_type: PacketType) {
        self.write_byte(packet_type as u8);
    }

    pub fn write_uint32(&mut self, value: u32) {
        self.write_bytes(value.to_be_bytes().to_vec());
    }

    pub fn write_mpint(&mut self, value: Vec<u8>) {
        todo!()
    }

    pub fn write_bytes(&mut self, bytes: Vec<u8>) {
        self.write_uint32(bytes.len() as u32);
        self.write_raw_bytes(bytes);
    }

    pub fn write_raw_bytes(&mut self, bytes: Vec<u8>) {
        self.data.write(bytes.as_ref()).unwrap();
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.data.write(&[byte]).unwrap();
    }

    pub fn write_string(&mut self, string: String) {
        let bytes = string.as_bytes().to_vec();
        self.write_bytes(bytes);
    }

    pub fn write_string_list(&mut self, string_list: Vec<String>) {
        let string = string_list.join(",");
        self.write_string(string);
    }
}
use std::io::{Cursor, Read};

pub struct ByteReader {
    data: Cursor<Vec<u8>>,
}

impl ByteReader {
    pub fn from(data: Vec<u8>) -> Self {
        Self {
            data: Cursor::new(data),
        }
    }

    pub fn get_bytes(&mut self, length: usize) -> Vec<u8> {
        let mut result = vec![0u8; length];
        self.data.read_exact(&mut result).unwrap();
        result
    }

    pub fn get_byte(&mut self) -> u8 {
        self.get_bytes(1)[0]
    }

    pub fn get_boolean(&mut self) -> bool {
        return self.get_bytes(1)[0] != 0;
    }

    pub fn get_uint32(&mut self) -> u32 {
        let bytes = self.get_bytes(4);
        return u32::from_be_bytes(bytes.try_into().unwrap());
    }

    pub fn get_string(&mut self) -> String {
        let length = self.get_uint32() as usize;
        if length == 0 {
            return String::new();
        }
        let bytes = self.get_bytes(length);
        return String::from_utf8(bytes).unwrap();
    }

    pub fn get_namelist(&mut self) -> Vec<String> {
        self.get_string()
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    }

    pub fn get_mpint(&mut self) -> Vec<u8> {
        todo!()
    }
}

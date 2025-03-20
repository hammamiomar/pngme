use std::{fmt::{self, Error}, str::{self}};

use crate::chunk_type::{self, ChunkType};

use crc::{Crc, CRC_32_ISO_HDLC};

pub struct Chunk{
    data_length : [u8;4],
    chunk_type : ChunkType,
    message_bytes: Vec<u8>,
    crc_chunk: [u8;4],
}
impl Chunk {
    fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk{
        let data_length = u32::to_be_bytes(data.len() as u32);
        let mut crc_bytes = chunk_type.bytes().to_vec();
        crc_bytes.extend_from_slice(&data);
        
        // Use the standard CRC-32 algorithm
        const CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let crc = CRC.checksum(&crc_bytes);
        let crc_chunk = u32::to_be_bytes(crc);
        
        Chunk { 
            data_length, 
            chunk_type, 
            message_bytes: data, 
            crc_chunk 
        }
    }
    fn length(&self) -> u32{
        u32::from_be_bytes(self.data_length)
    }
    fn chunk_type(&self) -> &ChunkType{
        &self.chunk_type
    }
    fn data(&self) -> &[u8]{
        &self.message_bytes
    }
    fn crc(&self) -> u32{
        u32::from_be_bytes(self.crc_chunk)
    }
    fn data_as_string(&self) -> Result<String,Error>{
        match str::from_utf8(&self.message_bytes[..]) {
            Ok(s) => Ok(s.to_string()),
            Err(_) => Err(fmt::Error),
        }
    }
    fn as_bytes(&self) -> Vec<u8>{
        self.data_length
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.message_bytes.iter())
            .chain(self.crc_chunk.iter())
            .copied()
            .collect()
    }
}
impl TryFrom<&[u8]> for Chunk{
    type Error = &'static str;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 12 {  // 4 (length) + 4 (type) + 4 (crc) minimum
            return Err("Invalid chunk size");
        }

        // Extract parts of the chunk
        let data_length = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        
        // Validate length matches the actual data
        if bytes.len() != (data_length as usize + 12) {
            return Err("Invalid chunk size");
        }
        
        // Extract chunk type
        let chunk_type_bytes = [bytes[4], bytes[5], bytes[6], bytes[7]];
        let chunk_type = match ChunkType::try_from(chunk_type_bytes) {
            Ok(ct) => ct,
            Err(e) => return Err("ChunkError::InvalidChunkType(e)"),
        };
        
        // Extract message bytes
        let message_start = 8;
        let message_end = message_start + data_length as usize;
        let message_bytes = bytes[message_start..message_end].to_vec();
        
        // Extract CRC
        let crc_bytes = [
            bytes[message_end], 
            bytes[message_end + 1],
            bytes[message_end + 2],
            bytes[message_end + 3]
        ];
        let actual_crc = u32::from_be_bytes(crc_bytes);
        
        // Calculate expected CRC
        let mut crc_bytes = chunk_type_bytes.to_vec();
        crc_bytes.extend_from_slice(&message_bytes);
        
        const CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let expected_crc = CRC.checksum(&crc_bytes);
        
        // Validate CRC
        if expected_crc != actual_crc {
            return Err("ChunkError::InvalidCrc(expected_crc, actual_crc)");
        }
        
        Ok(Chunk {
            data_length: data_length.to_be_bytes(),
            chunk_type,
            message_bytes,
            crc_chunk: actual_crc.to_be_bytes(),
        })
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = self.data_as_string();

        let display = match string{
            Ok(string) => string,
            Err(_) => return Err(fmt::Error)
        };
        write!(f,"{}",display)

    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        let _chunk_string = format!("{}", chunk);
    }
}


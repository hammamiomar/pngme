use std::{fmt, str::{self}};


#[derive(PartialEq, Eq, Debug)]
pub struct ChunkType{
   code: [u8;4],
}
impl ChunkType{
    fn bytes(&self) -> [u8;4]{
        self.code
    }

    fn is_valid(&self) -> bool{
        let array = &self.code;
        let valid = array.iter().all(|&i| i.is_ascii_lowercase() || i.is_ascii_uppercase());
        valid && array[2].is_ascii_uppercase()
    }

    fn is_critical(&self) -> bool{
        self.code[0].is_ascii_uppercase()
    }

    fn is_public(&self) -> bool{
        self.code[1].is_ascii_uppercase()
    }

    fn is_reserved_bit_valid(&self) -> bool{
        self.code[2].is_ascii_uppercase()
    }
    fn is_safe_to_copy(&self) -> bool{
        self.code[3].is_ascii_lowercase()
    }
}
impl TryFrom<[u8;4]> for ChunkType {
    type Error = &'static str;
    fn try_from(input: [u8;4]) -> Result<Self,Self::Error>{
        if input.iter().all(|&i| i.is_ascii_lowercase() || i.is_ascii_uppercase()){
            Ok(ChunkType { code: input })
        }else{
            Err("Only upper and lowercase!")
        }
        
    }
}

impl str::FromStr for ChunkType{
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();

        let chunk:[u8;4]= match bytes.try_into(){
            Ok(arr) => arr,
            Err(_) => return Err("Invalid length!"),
        };
        Self::try_from(chunk)
    }
}

impl fmt::Display for ChunkType{
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result{
        let binding = self.bytes();
        let string = str::from_utf8(&binding);
        let display = match string{
            Ok(string) => string,
            Err(_) => return Err(fmt::Error),
        };
        write!(f, "{}", display )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}

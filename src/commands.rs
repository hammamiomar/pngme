use std::path::Path;
use std::fs;
use std::str::FromStr;

use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::Result;

/// Encodes a message into a PNG file and saves the result
pub fn encode(
    file_path: &Path,
    chunk_type: &str,
    message: &str,
    output_path: Option<&Path>,
) -> Result<()> {
    // Read the PNG file
    let input_data = fs::read(file_path)?;
    let mut png = Png::try_from(input_data.as_slice())?;
    
    // Create chunk type
    let chunk_type = ChunkType::from_str(chunk_type)?;
    
    // Create chunk with message
    let data = message.as_bytes().to_vec();
    let chunk = Chunk::new(chunk_type, data);
    
    // Add chunk to PNG
    png.append_chunk(chunk);
    
    // Determine output path
    let out_path = match output_path {
        Some(path) => path,
        None => file_path,
    };
    
    // Save the modified PNG
    let png_bytes = png.as_bytes();
    fs::write(out_path, png_bytes)?;
    
    println!("Message encoded successfully to {}", out_path.display());
    Ok(())
}

/// Decodes a message from a PNG file
pub fn decode(file_path: &Path, chunk_type: &str) -> Result<String> {
    // Read the PNG file
    let input_data = fs::read(file_path)?;
    let png = Png::try_from(input_data.as_slice())?;
    
    // Find the chunk with the given type
    match png.chunk_by_type(chunk_type) {
        Some(chunk) => {
            let message = chunk.data_as_string()?;
            println!("Decoded message: {}", message);
            Ok(message)
        },
        None => Err(format!("Chunk type '{}' not found", chunk_type).into()),
    }
}

/// Removes a chunk from a PNG file
pub fn remove(file_path: &Path, chunk_type: &str) -> Result<()> {
    // Read the PNG file
    let input_data = fs::read(file_path)?;
    let mut png = Png::try_from(input_data.as_slice())?;
    
    // Remove the chunk
    let chunk = png.remove_first_chunk(chunk_type)?;
    
    // Save the modified PNG
    let png_bytes = png.as_bytes();
    fs::write(file_path, png_bytes)?;
    
    println!("Removed chunk '{}' from {}", chunk_type, file_path.display());
    println!("Removed chunk contained: {}", chunk.data_as_string()?);
    
    Ok(())
}

/// Prints all chunks in a PNG file
pub fn print_chunks(file_path: &Path) -> Result<()> {
    // Read the PNG file
    let input_data = fs::read(file_path)?;
    let png = Png::try_from(input_data.as_slice())?;
    
    // Print the PNG information
    println!("{}", png);
    
    Ok(())
}
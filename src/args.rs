use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Encode a message into a PNG file
    Encode {
        /// Path to the PNG file
        #[arg(short, long)]
        file: PathBuf,
        
        /// Chunk type (4 characters)
        #[arg(short, long)]
        chunk_type: String,
        
        /// Message to encode
        #[arg(short, long)]
        message: String,
        
        /// Output file path (if not provided, will overwrite the input file)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    
    /// Decode a message from a PNG file
    Decode {
        /// Path to the PNG file
        #[arg(short, long)]
        file: PathBuf,
        
        /// Chunk type to look for
        #[arg(short, long)]
        chunk_type: String,
    },
    
    /// Remove a chunk from a PNG file
    Remove {
        /// Path to the PNG file
        #[arg(short, long)]
        file: PathBuf,
        
        /// Chunk type to remove
        #[arg(short, long)]
        chunk_type: String,
    },
    
    /// Print all chunks in a PNG file
    Print {
        /// Path to the PNG file
        #[arg(short, long)]
        file: PathBuf,
    },
}

// Implement a function to parse args for testing/usage in commands.rs
pub fn parse_args() -> Args {
    Args::parse()
}
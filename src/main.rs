mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use args::{parse_args, Commands};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = parse_args();

    match args.command {
        Commands::Encode { file, chunk_type, message, output } => {
            commands::encode(&file, &chunk_type, &message, output.as_deref())?;
        }
        Commands::Decode { file, chunk_type } => {
            commands::decode(&file, &chunk_type)?;
        }
        Commands::Remove { file, chunk_type } => {
            commands::remove(&file, &chunk_type)?;
        }
        Commands::Print { file } => {
            commands::print_chunks(&file)?;
        }
    }

    Ok(())
}
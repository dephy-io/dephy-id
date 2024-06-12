pub mod assertions;
pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;
pub mod utils;
pub mod pda;

pub use solana_program;

solana_program::declare_id!("D6QAaYRpbRXhWicCbF6knEGmoLXiAvG8MsT77ZtKNo3K");
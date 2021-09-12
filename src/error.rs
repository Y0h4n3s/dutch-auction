use thiserror::Error;
use solana_program::program_error::ProgramError;
pub type AuctionResult<T = ()> = Result<T, AuctionError>;
#[derive(Error, Debug, PartialEq, Eq)]
pub enum AuctionError {

}



pub type AuctionInstructionResult<T> = Result<T, AuctionInstructionError>;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum AuctionInstructionError {
    #[error("Invalid Instruction Data")]
    InvalidInstruction,
    #[error("Start Timestamp Cannot be less than end timestamp")]
    InvalidTimeRange
}


impl From<ProgramError> for AuctionInstructionError {
    fn from(err: ProgramError) -> Self {
        AuctionInstructionError::from(err)
    }
}

impl From<ProgramError> for AuctionError {
    fn from(err: ProgramError) -> Self {
        AuctionError::from(err)
    }
}

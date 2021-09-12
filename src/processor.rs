use solana_program::pubkey::Pubkey;
use solana_program::account_info::AccountInfo;
use crate::error::AuctionResult;
use crate::instruction::DutchAuctionInstruction;
use solana_program::program_error::ProgramError;
use crate::instruction::{CreateAuction, PlaceBid};

use solana_program::msg;
use crate::PREFIX;
pub struct Processor{}

impl Processor {
    pub fn process(program_id: &Pubkey,  accounts: &[AccountInfo], data: &[u8]) -> AuctionResult {
        msg!("{:?}", data);
        let instruction = DutchAuctionInstruction::unpack(data).ok_or(ProgramError::InvalidInstructionData).unwrap();

        match instruction {
            DutchAuctionInstruction::CreateAuction(args) => {
                Self::process_create_dutch_auction(program_id, accounts, args)?
            },
            DutchAuctionInstruction::PlaceBid(args) => {
                Self::process_place_bin_auciton(program_id, accounts, args)?
            }
        }
        Ok(())
    }


    pub fn process_create_dutch_auction(program_id: &Pubkey, accounts: &[AccountInfo], auction_data: CreateAuction ) -> AuctionResult {
        let auction_path = [
            PREFIX.as_bytes(),
            program_id.as_ref(),
            &auction_data.auction_item.to_bytes(),
        ];
        let (auction_account, bump) = Pubkey::find_program_address(&auction_path, program_id);
        msg!("{:?}, {:?}", auction_account, auction_data);
        // store auction ...
        Ok(())
    }

    pub fn process_place_bin_auciton(program_id: &Pubkey, accounts: &[AccountInfo], bid_data: PlaceBid) -> AuctionResult {
        // get the auciton and place a bid on it
        Ok(())
    }
}
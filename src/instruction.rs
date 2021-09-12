use crate::error::{AuctionInstructionResult, AuctionInstructionError};
use solana_program::clock::Clock;
use solana_program::msg;
use solana_program::sysvar::Sysvar;
use solana_program::pubkey::Pubkey;
use arrayref::{array_refs, array_ref};
use borsh::{BorshDeserialize, BorshSerialize};


#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
struct CreateAuctionArgs {
    duration: u64,
    amount_of_tokens: u64,
    start_price: u64,
    reserve_price: u64,
    pub(crate) auction_item: Pubkey,
}

#[derive(Debug)]
pub struct CreateAuction {
    duration: u64,
    amount_of_tokens: u64,
    start_price: u64,
    reserve_price: u64,
    start_timestamp: u128,
    end_timestamp: u128,
    pub(crate) auction_item: Pubkey,
    fee: u64,

}


#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
struct PlaceBidArgs {
    amount_of_tokens: u64,
    bid_amount: u64,

}
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub(crate) struct PlaceBid {
    amount_of_tokens: u64,
    bid_amount: u64,

}

impl CreateAuction {
    pub fn unpack(data: &[u8; 64]) -> AuctionInstructionResult<Self> {

        let decoded = CreateAuctionArgs::try_from_slice(data).unwrap();
        let start_timestamp = Clock::get()?.unix_timestamp as u128;
        let end_timestamp = start_timestamp + decoded.duration as u128;
        let fee = 100 as u64;
        if start_timestamp > end_timestamp {
            return Err(AuctionInstructionError::InvalidTimeRange)
        }
        Ok(CreateAuction {
            duration: decoded.duration,
            amount_of_tokens: decoded.amount_of_tokens,
            start_price: decoded.start_price,
            reserve_price: decoded.reserve_price,
            start_timestamp,
            end_timestamp,
            auction_item: decoded.auction_item,
            fee
        })
    }
}

impl PlaceBid {
    pub fn unpack(data: &[u8]) {

    }
}
pub enum DutchAuctionInstruction{

    CreateAuction(CreateAuction),

    PlaceBid(PlaceBid)
}

impl DutchAuctionInstruction {
    pub fn unpack(data: &[u8]) -> Option<Self> {
        let (&[instruction_id], data) = array_refs![data, 1; ..;];

        match instruction_id {
            0 => {
                let data_arr = array_ref![data, 0, 64];
                Some(Self::CreateAuction(CreateAuction::unpack(data_arr).unwrap()))
            },
            1 => {
                let data_arr = array_ref![data, 0, 64];
                Some(Self::PlaceBid(PlaceBid::unpack(data_arr).unwrap()))
            }
            _ => {None}
        }
    }
}
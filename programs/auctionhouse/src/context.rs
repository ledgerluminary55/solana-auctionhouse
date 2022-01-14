use anchor_lang::prelude::*;
use anchor_lang::solana_program::{system_program};
use crate::account::*;
use crate::utils::*;

#[derive(Accounts)]
#[instruction(bump: u8, title: String, floor: u64, increment: u64, start_time: u64, end_time: u64, bidder_cap: u64)]
pub struct CreateAuction<'info> {
    #[account(init,
        seeds=[b"auction", owner.to_account_info().key.as_ref(), name_seed(&title)],
        bump = bump,
        payer = owner,
        space = Auction::LEN +
        VECTOR_LENGTH_PREFIX + (bidder_cap as usize)*PUBLIC_KEY_LENGTH +
        VECTOR_LENGTH_PREFIX + (bidder_cap as usize)*U64_LENGTH)]
    pub auction: Account<'info, Auction>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CancelAuction<'info> {
    #[account(mut, has_one = owner)]
    pub auction: Account<'info, Auction>,
    pub owner: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct MakeBid<'info> {
    #[account(mut)]
    pub auction: Account<'info, Auction>,
    #[account(mut)]
    pub bidder: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ReclaimBid<'info> {
    #[account(mut)]
    pub auction: Account<'info, Auction>,
    #[account(mut)]
    pub bidder: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct WithdrawItem<'info> {
    #[account(mut)]
    pub auction: Account<'info, Auction>,
    #[account(mut)]
    pub winner: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct WithdrawWinningBid<'info> {
    #[account(mut, has_one = owner)]
    pub auction: Account<'info, Auction>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ReclaimItem<'info> {
    #[account(mut, has_one = owner)]
    pub auction: Account<'info, Auction>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
}
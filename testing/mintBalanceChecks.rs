mod constants;
mod error;

use anchor_lang::{
    prelude::*,
    solana_program::{clock::Clock, hash::hash, program::invoke, system_instruction::transfer},
};
use anchor_spl::token::{self, Mint, TokenAccount};
use crate::{constants::*, error::LotteryError};

declare_id!("EiUfFXAuMWipRafPtoz2AWHmikzmE8qNdhGESnRQWLDB");

#[program]
mod lottery {
    use super::*;

    pub fn acc_balance(ctx: Context<AccBalance>, lottery_id: u32) -> Result<()> {
        let ticket = &mut ctx.accounts.bal_to_tickets;
        let token_amount = ctx.accounts.payer_token_account.amount; // pulls token account balance
        let check_mint = ctx.accounts.payer_token_account.mint; // pulls associated mint, should match "token_mint"

        // Check if the mint address of the token account matches the desired mint address
        if check_mint != *ctx.accounts.desired_mint.to_account_info().key {
            // if *check_mint != ctx.accounts.desired_mint.key() {
            return Err(LotteryError::InvalidMint.into()); // Return an error if the mint address doesn't match
        }

        ticket.tickets = token_amount;

        msg!("token_amount: {}", token_amount);
        msg!("check_mint: {}", check_mint);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct AccBalance<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 8,
        seeds = [
            payer.key().as_ref(),
        ],
        bump,
    )]
    pub bal_to_tickets: Account<'info, BalToTickets>,
    #[account(mut)]
    pub payer_token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub desired_mint: Account<'info, Mint>, // Account representing the desired mint address
    pub system_program: Program<'info, System>,
}

#[account]
pub struct BalToTickets {
    pub tickets: u64,
}

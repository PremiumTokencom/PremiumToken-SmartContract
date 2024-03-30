mod constants;
mod error;

use anchor_lang::{
    prelude::*,
    solana_program::{clock::Clock, hash::hash, program::invoke, system_instruction::transfer},
};
use anchor_spl::token::{self, Mint, TokenAccount};
use crate::{constants::*, error::LotteryError};

declare_id!("HwmXHjeM386CqG34wcZKCqpANCJmCSCQd5J1NANudnWW");

#[program]
mod lottery {
    use super::*;
    pub fn init_master(_ctx: Context<InitMaster>) -> Result<()> {
        Ok(())
    }

    pub fn create_lottery(
        ctx: Context<CreateLottery>,
        ticket_price: u64,
        token_key: Pubkey,
    ) -> Result<()> {
        let master = &mut ctx.accounts.master;
        let lottery = &mut ctx.accounts.lottery;

        // Increment the last ticket id
        master.last_id += 1;

        // Set lottery values
        lottery.id = master.last_id;
        lottery.authority = ctx.accounts.authority.key();
        lottery.ticket_price = ticket_price;
        lottery.token = token_key;

        msg!("Created lottery: {}", lottery.id);
        msg!("Authority: {}", lottery.authority);
        msg!("Ticket price: {}", lottery.ticket_price);

        Ok(())
    }

    pub fn pick_winner(ctx: Context<PickWinner>, _lottery_id: u32) -> Result<()> {
        let lottery = &mut ctx.accounts.lottery;

        if lottery.winner_id.is_some() {
            return err!(LotteryError::WinnerAlreadyExists);
        }
        if lottery.last_ticket_id == 0 {
            return err!(LotteryError::NoTickets);
        }

        // Pick a pseudo-random winner
        let clock = Clock::get()?;
        let pseudo_random_number = ((u64::from_le_bytes(
            <[u8; 8]>::try_from(&hash(&clock.unix_timestamp.to_be_bytes()).to_bytes()[..8])
                .unwrap(),
        ) * clock.slot)
            % u64::MAX) as u64;

        let winner_id = (pseudo_random_number % lottery.last_ticket_id) + 1;
        lottery.winner_id = Some(winner_id);

        msg!("Winner id: {}", pseudo_random_number);

        Ok(())
    }

    pub fn acc_balance(ctx: Context<AccBalance>, lottery_id: u32) -> Result<()> {
        let lottery = &mut ctx.accounts.lottery;
        let ticket = &mut ctx.accounts.bal_to_tickets;
        let buyer = &ctx.accounts.payer;
        let token_amount = ctx.accounts.token_account.amount; // pulls token account balance
        let check_mint = ctx.accounts.token_account.mint; // pulls associated mint, should match "token_mint"

        // Check if the mint address of the token account matches the desired mint address
        if check_mint != lottery.token {
            // if *check_mint != ctx.accounts.desired_mint.key() {
            return Err(LotteryError::InvalidMint.into()); // Return an error if the mint address doesn't match
        }

        let tokens_per_ticket = 1000000000000; // token to ticket ratio * decimals
        if token_amount <= tokens_per_ticket {
            return Err(LotteryError::InvalidMint.into());
        }

        ticket.tickets = token_amount;
        ticket.id = lottery.last_ticket_id + 1;
        ticket.lottery_id = lottery_id;
        ticket.authority = ctx.accounts.token_account.key();

        //ticket from and to
        ticket.ticketFrom = ticket.id;
        ticket.ticketTo = ticket.ticketFrom + (token_amount / tokens_per_ticket);
        lottery.last_ticket_id = ticket.ticketTo;

        msg!("token_amount: {}", token_amount);
        msg!("check_mint: {}", check_mint);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMaster<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 4,
        seeds = [MASTER_SEED.as_bytes()],
        bump,
    )]
    pub master: Account<'info, Master>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Master {
    pub last_id: u32,
}

#[derive(Accounts)]
pub struct CreateLottery<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 4 + 32 + 8 + 8 + 1 + 8 + 1 + 32,
        seeds = [LOTTERY_SEED.as_bytes(), &(master.last_id + 1).to_le_bytes()],
        bump,
    )]
    pub lottery: Account<'info, Lottery>,
    #[account(
        mut,
        seeds = [MASTER_SEED.as_bytes()],
        bump,
    )]
    pub master: Account<'info, Master>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Lottery {
    pub id: u32,
    pub authority: Pubkey,
    pub ticket_price: u64,
    pub last_ticket_id: u64,
    pub winner_id: Option<u64>,
    pub claimed: bool,
    pub token: Pubkey,
}

#[derive(Accounts)]
#[instruction(lottery_id: u32)]
pub struct PickWinner<'info> {
    #[account(
        mut,
        seeds = [LOTTERY_SEED.as_bytes(), &lottery_id.to_le_bytes()],
        bump,
        has_one = authority,
    )]
    pub lottery: Account<'info, Lottery>,
    pub authority: Signer<'info>,
}
#[derive(Accounts)]
#[instruction(lottery_id: u32)]
pub struct AccBalance<'info> {
    #[account(
        mut,
        seeds = [LOTTERY_SEED.as_bytes(), &lottery_id.to_le_bytes()],
        bump,
    )]
    pub lottery: Account<'info, Lottery>,
    #[account(
        init,
        payer = payer,
        space = 8 + 8 + 8 + 8 + 8 + 4 + 32,
        seeds = [
            TICKET_SEED.as_bytes(),
            lottery.key().as_ref(),
            &(lottery.last_ticket_id + 1).to_le_bytes(),
            token_account.key().as_ref(),
        ],
        bump,
    )]
    pub bal_to_tickets: Account<'info, BalToTickets>,
    #[account(mut)]
    pub token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct BalToTickets {
    pub tickets: u64,
    pub ticketFrom: u64,
    pub ticketTo: u64,
    pub id: u64,
    pub lottery_id: u32,
    pub authority: Pubkey,
}

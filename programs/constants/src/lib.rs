mod constants;
mod error;

use anchor_lang::{
    prelude::*,
    solana_program::{clock::Clock, hash::hash},
};
use anchor_spl::token::{self, TokenAccount};
use crate::{constants::*, error::TokenAutomaticDrawError};

declare_id!("AxtuDAYigkzFLSjR9WE7jHMCvetVDaAuE2GQgkoHWFQa");

#[program]
mod token_automatic_draw {
    use super::*;

    pub fn initialize_master(_ctx: Context<InitializeMaster>) -> Result<()> {
        let master = &mut _ctx.accounts.master;
        let payer = &_ctx.accounts.payer;

        master.authority = *payer.key;
        Ok(())
    }

    pub fn create_automatic_draw(
        ctx: Context<CreateAutomaticDraw>,
        tokens_per_entry: u64,
        token_mint: Pubkey,
    ) -> Result<()> {
        let master = &mut ctx.accounts.master;
        let automatic_draw = &mut ctx.accounts.automatic_draw;

        // Increment the last automatic_draw id
        master.last_id += 1;

        // Set automatic_draw values
        automatic_draw.id = master.last_id;
        automatic_draw.authority = ctx.accounts.authority.key();
        automatic_draw.token_mint = token_mint;
        automatic_draw.tokens_per_entry = tokens_per_entry;

        msg!("Created automatic_draw: {}", automatic_draw.id);
        msg!("Authority: {}", automatic_draw.authority);
        msg!("Entry cost: {}", automatic_draw.tokens_per_entry);

        Ok(())
    }

    //Draws one winner from participants based on their token holdings at each market cap milestone of $10,000,000.
    pub fn select_winner(ctx: Context<SelectWinner>, _automatic_draw_id: u32) -> Result<()> {
        let automatic_draw = &mut ctx.accounts.automatic_draw;

        if automatic_draw.winner_id.is_some() {
            return Err(TokenAutomaticDrawError::WinnerAlreadySelected.into());
        }
        if automatic_draw.last_entry_id == 0 {
            return Err(TokenAutomaticDrawError::NoEntries.into());
        }

        // Select a pseudo-random winner
        let clock = Clock::get()?;
        let pseudo_random_number = ((u64::from_le_bytes(
            <[u8; 8]>::try_from(&hash(&clock.unix_timestamp.to_be_bytes()).to_bytes()[..8])
                .unwrap(),
        ).wrapping_mul(clock.slot))
            % u64::MAX) as u64;

        let winner_id = (pseudo_random_number % automatic_draw.last_entry_id) + 1;
        automatic_draw.winner_id = Some(winner_id);

        msg!("Winner id: {}", pseudo_random_number);

        Ok(())
    }

    pub fn enter_automatic_draw(
        ctx: Context<EnterAutomaticDraw>,
        automatic_draw_id: u32,
    ) -> Result<()> {
        let automatic_draw = &mut ctx.accounts.automatic_draw;
        let entry = &mut ctx.accounts.holding_to_entries;

        if automatic_draw.winner_id.is_some() {
            return Err(TokenAutomaticDrawError::WinnerAlreadySelected.into());
        }

        let token_holdings = ctx.accounts.token_account.amount;
        let mint_address = ctx.accounts.token_account.mint;

        // Check if the mint address of the token account matches the desired mint address
        if mint_address != automatic_draw.token_mint {
            return Err(TokenAutomaticDrawError::InvalidMint.into());
        }

        if token_holdings <= automatic_draw.tokens_per_entry {
            return Err(TokenAutomaticDrawError::InsufficientTokens.into());
        }

        for address in EXCLUDED_ADDRESSES.iter() {
            if *address == ctx.accounts.token_account.key() {
                msg!("excluded address found: {:?}", address);
                return Err(TokenAutomaticDrawError::ExcludedAddress.into());
            }
        }

        entry.holdings = token_holdings;
        entry.automatic_draw_id = automatic_draw_id;
        entry.entered_by = ctx.accounts.token_account.key();

        // Assign entry range
        entry.entry_from = automatic_draw.last_entry_id + 1;
        entry.entry_to =
            automatic_draw.last_entry_id + (token_holdings / automatic_draw.tokens_per_entry);
        automatic_draw.last_entry_id = entry.entry_to;

        msg!("Token holdings: {}", token_holdings);
        msg!("Mint address: {}", mint_address);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeMaster<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 4 + 32,
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
    pub authority: Pubkey,
}

#[derive(Accounts)]
pub struct CreateAutomaticDraw<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 4 + 32 + 8 + 8 + 1 + 8 + 1 + 32,
        seeds = [AUTOMATIC_DRAW_SEED.as_bytes(), &(master.last_id + 1).to_le_bytes()],
        bump,
    )]
    pub automatic_draw: Account<'info, AutomaticDraw>,
    #[account(
        mut,
        seeds = [MASTER_SEED.as_bytes()],
        bump,
    )]
    pub master: Account<'info, Master>,
    #[account(mut,address=master.authority)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct AutomaticDraw {
    pub id: u32,
    pub authority: Pubkey,
    pub last_entry_id: u64,
    pub tokens_per_entry: u64,
    pub winner_id: Option<u64>,
    pub token_mint: Pubkey,
}

#[derive(Accounts)]
#[instruction(automatic_draw_id: u32)]
pub struct SelectWinner<'info> {
    #[account(
        mut,
        seeds = [AUTOMATIC_DRAW_SEED.as_bytes(), &automatic_draw_id.to_le_bytes()],
        bump,
        has_one = authority,
    )]
    pub automatic_draw: Account<'info, AutomaticDraw>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(automatic_draw_id: u32)]
pub struct EnterAutomaticDraw<'info> {
    #[account(
        mut,
        seeds = [AUTOMATIC_DRAW_SEED.as_bytes(), &automatic_draw_id.to_le_bytes()],
        bump,
    )]
    pub automatic_draw: Account<'info, AutomaticDraw>,
    #[account(
        init,
        payer = payer,
        space = 8 + 8 + 8 + 8 + 4 + 32,
        seeds = [
            ENTRY_SEED.as_bytes(),
            automatic_draw.key().as_ref(),
            token_account.key().as_ref(),
        ],
        bump,
    )]
    pub holding_to_entries: Account<'info, Entry>,
    #[account(mut)]
    pub token_account: Account<'info, token::TokenAccount>,
    #[account(mut,address=automatic_draw.authority)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Entry {
    pub holdings: u64,
    pub entry_from: u64,
    pub entry_to: u64,
    pub automatic_draw_id: u32,
    pub entered_by: Pubkey,
}

mod constants;
mod error;

use anchor_lang::{
    prelude::*,
    solana_program::{clock::Clock, hash::hash},
};
use anchor_spl::token::{self, TokenAccount};
use crate::{constants::*, error::TokenCampaignError};
declare_id!("651iXWmZSZhJ4qmBFtyXJNffpJuBF8B1TNQyjz5SxeTi");

#[program]
mod token_campaign {
    use super::*;

    pub fn initialize_master(_ctx: Context<InitializeMaster>) -> Result<()> {
        let master = &mut _ctx.accounts.master;
        let payer = &_ctx.accounts.payer;

        master.authority = *payer.key;
        Ok(())
    }

    pub fn create_campaign(
        ctx: Context<CreateCampaign>,
        tokens_per_entry: u64,
        token_mint: Pubkey,
    ) -> Result<()> {
        let master = &mut ctx.accounts.master;
        let campaign = &mut ctx.accounts.campaign;

        // Increment the last campaign id
        master.last_id += 1;

        // Set campaign values
        campaign.id = master.last_id;
        campaign.authority = ctx.accounts.authority.key();
        campaign.token_mint = token_mint;
        campaign.tokens_per_entry = tokens_per_entry;

        msg!("Created campaign: {}", campaign.id);
        msg!("Authority: {}", campaign.authority);
        msg!("Entry cost: {}", campaign.tokens_per_entry);

        Ok(())
    }

    //Draws one winner from participants based on their token holdings at each market cap milestone of $10,000,000. 
    pub fn select_winner(ctx: Context<SelectWinner>, _campaign_id: u32) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;

        if campaign.winner_id.is_some() {
            return Err(TokenCampaignError::WinnerAlreadySelected.into());
        }
        if campaign.last_entry_id == 0 {
            return Err(TokenCampaignError::NoEntries.into());
        }

        // Select a pseudo-random winner
        let clock = Clock::get()?;
        let pseudo_random_number = ((u64::from_le_bytes(
            <[u8; 8]>::try_from(&hash(&clock.unix_timestamp.to_be_bytes()).to_bytes()[..8])
                .unwrap(),
        ) * clock.slot)
            % u64::MAX) as u64;

        let winner_id = (pseudo_random_number % campaign.last_entry_id) + 1;
        campaign.winner_id = Some(winner_id);

        msg!("Winner id: {}", pseudo_random_number);

        Ok(())
    }

    pub fn enter_campaign(ctx: Context<EnterCampaign>, campaign_id: u32) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;
        let entry = &mut ctx.accounts.holding_to_entries;

        if campaign.winner_id.is_some() {
            return Err(TokenCampaignError::WinnerAlreadySelected.into());
        }

        let token_holdings = ctx.accounts.token_account.amount;
        let mint_address = ctx.accounts.token_account.mint;

        // Check if the mint address of the token account matches the desired mint address
        if mint_address != campaign.token_mint {
            return Err(TokenCampaignError::InvalidMint.into());
        }

        if token_holdings <= campaign.tokens_per_entry {
            return Err(TokenCampaignError::InsufficientTokens.into());
        }

        entry.holdings = token_holdings;
        entry.campaign_id = campaign_id;
        entry.entered_by = ctx.accounts.token_account.key();

        // Assign entry range
        entry.entry_from = campaign.last_entry_id + 1;
        entry.entry_to = campaign.last_entry_id + (token_holdings / campaign.tokens_per_entry);
        campaign.last_entry_id = entry.entry_to;

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
pub struct CreateCampaign<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 4 + 32 + 8 + 8 + 1 + 8 + 1 + 32,
        seeds = [CAMPAIGN_SEED.as_bytes(), &(master.last_id + 1).to_le_bytes()],
        bump,
    )]
    pub campaign: Account<'info, Campaign>,
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
pub struct Campaign {
    pub id: u32,
    pub authority: Pubkey,
    pub last_entry_id: u64,
    pub tokens_per_entry: u64,
    pub winner_id: Option<u64>,
    pub token_mint: Pubkey,
}

#[derive(Accounts)]
#[instruction(campaign_id: u32)]
pub struct SelectWinner<'info> {
    #[account(
        mut,
        seeds = [CAMPAIGN_SEED.as_bytes(), &campaign_id.to_le_bytes()],
        bump,
        has_one = authority,
    )]
    pub campaign: Account<'info, Campaign>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(campaign_id: u32)]
pub struct EnterCampaign<'info> {
    #[account(
        mut,
        seeds = [CAMPAIGN_SEED.as_bytes(), &campaign_id.to_le_bytes()],
        bump,
    )]
    pub campaign: Account<'info, Campaign>,
    #[account(
        init,
        payer = payer,
        space = 8 + 8 + 8 + 8 + 4 + 32,
        seeds = [
            ENTRY_SEED.as_bytes(),
            campaign.key().as_ref(),
            &(campaign.last_entry_id + 1).to_le_bytes(),
            token_account.key().as_ref(),
        ],
        bump,
    )]
    pub holding_to_entries: Account<'info, Entry>,
    #[account(mut)]
    pub token_account: Account<'info, token::TokenAccount>,
    #[account(mut,address=campaign.authority)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Entry {
    pub holdings: u64,
    pub entry_from: u64,
    pub entry_to: u64,
    pub campaign_id: u32,
    pub entered_by: Pubkey,
}

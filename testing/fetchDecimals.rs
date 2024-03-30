/// TO FETCH THE DICIMAL NUMBER OF TOKEN
use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;
declare_id!("7BzEnRbxoX4fwzZaTPfkHCDrBBqsCxQajg2eWdkgagYk");

#[program]
pub mod balance {
    use super::*;

    pub fn read_balance(ctx: Context<ReadBalance>) -> Result<()> {
        // let balance = ctx.accounts.acct.to_account_info().lamports();
        let decimals = ctx.accounts.mint_x.decimals;

        // msg!("balance in Lamports is {}", balance);
        msg!("decimals of mint {}", decimals);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ReadBalance<'info> {
    /// CHECK: although we read this account's balance, we don't do anything with the information
    // pub acct: UncheckedAccount<'info>,
    pub mint_x: InterfaceAccount<'info, Mint>,
}
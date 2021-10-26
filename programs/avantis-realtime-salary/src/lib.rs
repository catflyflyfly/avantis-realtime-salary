use anchor_lang::prelude::*;

use anchor_lang::prelude::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Transfer};
use anchor_spl::token::{Mint, SetAuthority, TokenAccount};
use spl_token::instruction::AuthorityType;
use spl_token::solana_program::clock::UnixTimestamp;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

const SALARY_VAULT_PDA_SEED: &[u8] = b"salary_vault_authority";

#[program]
pub mod avantis_realtime_salary {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, _salary_vault_account_bump: u8) -> ProgramResult {
        // Transfer ownership of Salary's Vault to program
        let (salary_vault_authority, _bump) =
            Pubkey::find_program_address(&[SALARY_VAULT_PDA_SEED], ctx.program_id);

        token::set_authority(
            ctx.accounts.into_set_authority_context(),
            AuthorityType::AccountOwner,
            Some(salary_vault_authority),
        )?;
        Ok(())
    }

    pub fn add_employee(ctx: Context<AddEmployee>, daily_rate: u64, _bump: u8) -> ProgramResult {
        ctx.accounts.employee_salary_state.daily_rate = daily_rate;
        ctx.accounts.employee_salary_state.employee_pubkey = *ctx.accounts.employee.key;
        ctx.accounts.employee_salary_state.employee_token_account = ctx.accounts.employee_token_account.key();
        ctx.accounts.employee_salary_state.last_claimed_timestamp = Clock::get()?.unix_timestamp;

        Ok(())
    }

    pub fn claim_salary(ctx: Context<ClaimSalary>) -> ProgramResult {
        unimplemented!();
    }
}

#[derive(Accounts)]
#[instruction(salary_vault_account_bump: u8)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,

    #[account(
        init,
        seeds = [ b"salary_vault_account".as_ref()],
        bump = salary_vault_account_bump,
        payer = initializer,
        token::mint = mint,
        token::authority = initializer,
    )]
    pub vault_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub mint: Account<'info, Mint>,
}

impl<'info> Initialize<'info> {
    fn into_set_authority_context(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            account_or_mint: self.vault_account.to_account_info().clone(),
            current_authority: self.initializer.to_account_info().clone(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[derive(Accounts)]
#[instruction(daily_rate: u64, bump: u8)]
pub struct AddEmployee<'info> {
    #[account(mut)]
    pub adder: Signer<'info>,
    #[account(
        init,
        seeds = [employee.key.as_ref()],
        bump = bump,
        payer = adder,
        space = 8 + 32 + 32 + 8 + 8,
    )]
    pub employee_salary_state: Account<'info, EmployeeSalaryState>,
    pub employee_token_account: Account<'info, TokenAccount>,
    pub employee: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
pub struct EmployeeSalaryState {
    pub employee_pubkey: Pubkey,
    pub employee_token_account: Pubkey,
    pub daily_rate: u64,
    pub last_claimed_timestamp: i64,
}

#[derive(Accounts)]
pub struct ClaimSalary<'info> {
    pub claimer: Signer<'info>,
    pub employee_token_account: Account<'info, TokenAccount>,
    pub vault_account: Account<'info, TokenAccount>,
    pub pool_vault_authority: AccountInfo<'info>,
}

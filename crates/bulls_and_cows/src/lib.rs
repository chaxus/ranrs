use anchor_lang::prelude::*;
use solana_program::clock::Clock;

declare_id!("11111111111111111111111111111111");

#[program]
pub mod anchor_bac {
		use super::*;
		use std::cmp::Ordering;

		pub fn initialize(ctx: Context<AccountContext>) -> Result<()> {
				let guessing_account = &mut ctx.accounts.guessing_account;
				guessing_account.number = generate_random_number();
				Ok(())
		}
		
		pub fn guess(ctx: Context<AccountContext>, number: u32) -> Result<()> {
        let guessing_account = &mut ctx.accounts.guessing_account;
        let target = guessing_account.number;

        match number.cmp(&target) {
            Ordering::Less => {
                return err!(MyError::NumberTooSmall)
            }
            Ordering::Greater => {
                return  err!(MyError::NumberTooLarge);
            }
            Ordering::Equal => {
                return Ok(())
            }
        }
    }
}

fn generate_random_number() -> u32 {
		let clock = Clock::get().expect("Failed to get clock");
		let last_digit = (clock.unix_timestamp % 10) as u8;
		let result = (last_digit + 1) as u32;
		result
}

#[account]
pub struct GuessingAccount {
	pub number: u32,
}

#[derive(Accounts)]
pub struct AccountContext<'info> {
	
	#[account(
        init_if_needed,
        space=32,
        payer=payer,
        seeds = [b"guessing pda"],
        bump
    )]
	pub guessing_account: Account<'info, GuessingAccount>,

	#[account(mut)]
  pub payer: Signer<'info>,
	
	pub system_program: Program<'info, System>,
}

#[error_code]
pub enum MyError {
    #[msg("Too small")]
    NumberTooSmall,
    #[msg("Too larget")]
    NumberTooLarge
}
use anchor_lang::prelude::*;

declare_id!("Bs4SdFvqvYv2zndzEhNDdi1BUc9ffVjuNKpfy8Pf3P2a");

#[program]
pub mod messitributesolanacontract {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
      // Get a reference to the account.
      let base_account = &mut ctx.accounts.base_account;
      // Initialize total_gifs.
      base_account.total_videos = 0;
      Ok(())
  }

  	// Another function woo!
    pub fn add_video(ctx: Context<AddVideo>) -> ProgramResult {
      // Get a reference to the account and increment total_gifs.
      let base_account = &mut ctx.accounts.base_account;
      base_account.total_videos += 1;
      Ok(())
    }
}

// Attach certain variables to the StartStuffOff context.
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 10000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

// Specify what data you want in the AddVideo Context.
// Getting a handle on the flow of things :)?
#[derive(Accounts)]
pub struct AddVideo<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
}

// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {
    pub total_videos: u64,
}
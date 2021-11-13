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
    pub fn add_video(ctx: Context<AddVideo>, video_link: String) -> ProgramResult {
      // Get a reference to the account and increment total_gifs.
      let base_account = &mut ctx.accounts.base_account;
      let user = &mut ctx.accounts.user;

      // Build the struct.
        let item = ItemStruct {
          video_link: video_link.to_string(),
          user_address: *user.to_account_info().key,
        };
        
      // Add it to the gif_list vector.
      base_account.video_list.push(item);
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
  #[account(mut)]
  pub user: Signer<'info>,
}

// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub video_link: String,
    pub user_address: Pubkey,
}

// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {
    pub total_videos: u64,
    // Attach a Vector of type ItemStruct to the account.
    pub video_list: Vec<ItemStruct>,
}
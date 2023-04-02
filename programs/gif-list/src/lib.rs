use anchor_lang::prelude::*;

declare_id!("7hfWPsdmi52vhUTpcMGXGt6sXnrYorCuc5Pxj61mNNML");

#[program]
pub mod gif_list {
  use super::*;

  pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let account = &mut ctx.accounts.allowed_gifs;
    account.gifs = vec![];
    account.bump = *ctx.bumps.get("allowed_gifs").unwrap();
    Ok(())
  }

  // pub fn request_access_to_gif(ctx: Context<RequestAccess>) -> Result<()> {
  //   Ok(())
  // }
}

/* ACCOUNTS */

#[account]
pub struct AllowedGifs {
  pub gifs: Vec<String>,
  pub bump: u8
}

/* INSTRUCTION VALIDATION */

#[derive(Accounts)]
pub struct Initialize<'info> {
  #[account(mut)]
  pub authority: Signer<'info>,
  #[account(init,
    payer = authority,
    seeds = [b"allowed_gifs", authority.key().as_ref()],
    space = 9000,
    bump
  )]
  pub allowed_gifs: Account<'info, AllowedGifs>,
  pub system_program: Program<'info, System>
}

// #[derive(Accounts)]
// pub struct RequestAccess<'info> {
//   pub authority: Signer<'info>
//   #[account(seeds = [b"allowed_gifs", authority.key().as_ref()], bump = allowed_gifs.bump)]
//   pub allowed_gifs: Account<'info, AllowedGifs>
//   pub system_program: Program<'info, System>
// }

// impl<'info> RequestAccess

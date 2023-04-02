use anchor_lang::prelude::*;

declare_id!("9ZZFLXSdUMYcec7jj87AQWTr53VvALN36cVMTiohiKU");

#[program]
pub mod gif_list_admin {
  use super::*;

  pub fn initialize(ctx: Context<Initialize>, token_limit: u64) -> Result<()> {
    let gif_list_account = &mut ctx.accounts.gif_list;
    gif_list_account.gifs = vec![];
    gif_list_account.bump = *ctx.bumps.get("gif_list").unwrap();
    
    let conditions_account = &mut ctx.accounts.conditions;
    conditions_account.token_limit = token_limit;
    conditions_account.bump = *ctx.bumps.get("conditions").unwrap();
    Ok(())
  }

  pub fn add_gif(ctx: Context<AddGif>, gif: String) -> Result<()> {
    let account = &mut ctx.accounts.gif_list;
    account.gifs.push(gif);
    Ok(())
  }
}

/* ACCOUNTS */

#[account]
pub struct GifList {
  pub gifs: Vec<String>,
  pub bump: u8
}

#[account]
pub struct Conditions {
  pub token_limit: u64,
  pub bump: u8
}

/* INSTRUCTION VALIDATION */

#[derive(Accounts)]
pub struct Initialize<'info> {
  #[account(mut)]
  pub authority: Signer<'info>,
  #[account(init,
    payer = authority,
    seeds = [b"gif_list", authority.key().as_ref()],
    space = 9000,
    bump
  )]
  pub gif_list: Account<'info, GifList>,
  #[account(init,
    payer = authority,
    seeds = [b"conditions", authority.key().as_ref()],
    space = 9000,
    bump
  )]
  pub conditions: Account<'info, Conditions>,
  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct AddGif<'info> {
  pub authority: Signer<'info>,
  #[account(mut, seeds = [b"gif_list", authority.key().as_ref()], bump = gif_list.bump)]
  pub gif_list: Account<'info, GifList>,
  pub system_program: Program<'info, System>
}
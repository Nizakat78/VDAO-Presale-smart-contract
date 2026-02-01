use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, MintTo, Token, TokenAccount};

declare_id!("ReplaceWithYourProgramId1111111111111111111111111");

#[program]
pub mod vdao {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        _bump: u8,
    ) -> Result<()> {
        let cfg = &mut ctx.accounts.config;
        cfg.authority = *ctx.accounts.authority.key;
        cfg.mint = ctx.accounts.mint.key();
        cfg.team_token_account = ctx.accounts.team_token_account.key();
        cfg.bump = *ctx.bumps.get("config").unwrap_or(&0);
        Ok(())
    }

    pub fn mint_to_user(ctx: Context<MintToUser>, amount: u64) -> Result<()> {
        let seeds = &[b"config".as_ref(), &[ctx.accounts.config.bump]];
        let signer = &[&seeds[..]];

        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.recipient_token_account.to_account_info(),
            authority: ctx.accounts.program_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let mint_to_ctx = CpiContext::new_with_signer(cpi_program.clone(), cpi_accounts, signer);
        token::mint_to(mint_to_ctx, amount)?;

        let team_amount = amount / 10; // 10%

        let cpi_accounts_team = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.team_token_account.to_account_info(),
            authority: ctx.accounts.program_authority.to_account_info(),
        };
        let mint_to_team_ctx = CpiContext::new_with_signer(cpi_program.clone(), cpi_accounts_team, signer);
        token::mint_to(mint_to_team_ctx, team_amount)?;

        Ok(())
    }

    pub fn burn_from_user(ctx: Context<BurnFromUser>, amount: u64) -> Result<()> {
        let seeds = &[b"config".as_ref(), &[ctx.accounts.config.bump]];
        let signer = &[&seeds[..]];

        let cpi_accounts = Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.program_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let burn_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::burn(burn_ctx, amount)?;
        Ok(())
    }

    pub fn user_burn(ctx: Context<UserBurn>, amount: u64) -> Result<()> {
        let cpi_accounts = Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let burn_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::burn(burn_ctx, amount)?;
        Ok(())
    }
}

#[account]
pub struct Config {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub team_token_account: Pubkey,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 32 + 32 + 1, seeds = [b"config"], bump)]
    pub config: Account<'info, Config>,

    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub team_token_account: Account<'info, TokenAccount>,

    #[account(seeds = [b"config"], bump)]
    pub program_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct MintToUser<'info> {
    #[account(mut, has_one = mint)]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(seeds = [b"config"], bump = config.bump)]
    pub program_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub recipient_token_account: Account<'info, TokenAccount>,

    #[account(mut, address = config.team_token_account)]
    pub team_token_account: Account<'info, TokenAccount>,

    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct BurnFromUser<'info> {
    #[account(mut, has_one = mint)]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(seeds = [b"config"], bump = config.bump)]
    pub program_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct UserBurn<'info> {
    #[account(mut, has_one = mint)]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut, constraint = user_token_account.owner == *user.key)]
    pub user_token_account: Account<'info, TokenAccount>,

    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

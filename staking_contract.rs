// Note: This is a simplified version. Ensure to include proper error handling and security checks.
use anchor_lang::prelude::*;

declare_id!("Fg6S..."); // Replace with your actual Program ID

#[program]
pub mod yetis_staking {
    use super::*;

    pub fn stake_nft(ctx: Context<StakeNft>, nft_id: Pubkey) -> ProgramResult {
        let stake_account = &mut ctx.accounts.stake_account;
        stake_account.nft_id = nft_id;
        stake_account.staker = *ctx.accounts.staker.key;
        stake_account.stake_time = Clock::get().unwrap().unix_timestamp;
        Ok(())
    }

    // Additional functions for unstaking, rewards calculation, etc., will be added here.
}

#[derive(Accounts)]
pub struct StakeNft<'info> {
    #[account(init, payer = staker, space = 8 + 32 + 32 + 8)]
    pub stake_account: Account<'info, StakeAccount>,
    pub staker: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct StakeAccount {
    pub nft_id: Pubkey,
    pub staker: Pubkey,
    pub stake_time: i64,
}

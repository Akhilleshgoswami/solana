use anchor_lang::prelude::*;
use constants::*;
use instructions::*;
use state::*;
mod constants;
mod instructions;
mod state;
mod error;
declare_id!("DSBvNCUTNnGw2XRNWqKxzdgUJ1F5Cq6ZM9VVE83ttJHJ");

#[program]
pub mod stableccoin {
    use super::*;
    pub fn initialized_config(ctx: Context<InitializeConfig>) -> Result<()> {
        process_initialize_config(ctx)
    }
    pub fn update_config(ctx: Context<updateConfig>, min_health_factor: u64) -> Result<()> {
        process_update_config(ctx, min_health_factor)
    }
    pub fn deposit_collateral_and_mint_token(
        ctx: Context<DepositCollateralAndMintTokens>,
        amount_collateral: u64,
        amount_to_mint: u64,
    ) -> Result<()> {
        process_deposit_collateral_and_mint_token(ctx, amount_collateral, amount_to_mint)
    }
}

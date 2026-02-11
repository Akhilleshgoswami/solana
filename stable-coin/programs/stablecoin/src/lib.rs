use anchor_lang::prelude::*;
use constants::*;
use instructions::*;
use state::*;
mod constants;
mod error;
mod instructions;
mod state;
declare_id!("DSBvNCUTNnGw2XRNWqKxzdgUJ1F5Cq6ZM9VVE83ttJHJ");

#[program]
pub mod stableccoin {
    use super::*;
    pub fn initialized_config(ctx: Context<InitializeConfig>) -> Result<()> {
        process_initialize_config(ctx)
    }
    pub fn update_config(ctx: Context<UpdateConfig>, min_health_factor: u64) -> Result<()> {
        process_update_config(ctx, min_health_factor)
    }
    pub fn deposit_collateral_and_mint_token(
        ctx: Context<DepositCollateralAndMintTokens>,
        amount_collateral: u64,
        amount_to_mint: u64,
    ) -> Result<()> {
        process_deposit_collateral_and_mint_token(ctx, amount_collateral, amount_to_mint)
    }
    pub fn redeem_collateral_and_burn_tokens(
        ctx: Context<RedeemCollateralAndBurnTokens>,
        amount_collateral: u64,
        amount_to_burn: u64,
    ) -> Result<()> {
        process_redeem_collateral_and_burn_tokens(ctx, amount_collateral, amount_to_burn)
    }
    pub fn liquidate(ctx: Context<Liquidate>, amount_to_burn: u64) -> Result<()> {
        process_liquidate(ctx, amount_to_burn)
    }
}

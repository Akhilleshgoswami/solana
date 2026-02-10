use anchor_lang::prelude::*;
use anchor_spl::associated_token::spl_associated_token_account::solana_program::native_token::LAMPORTS_PER_SOL;
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

use crate::{
    constants::{FEED_ID, MAXIMUM_AGE, PRICE_FEED_DECIMAL_ADJUSTMENT},
    error::CustomError,
    state::{Collateral, Config},
};
// Check health factor for Collateral account is greater than minimum required health factor
pub fn check_health_factor(
    collateral: &Account<Collateral>,
    config: &Account<Config>,
    price_feed: &Account<PriceUpdateV2>,
) -> Result<()> {
    let health_factor = calculate_health_factor(collateral, config, price_feed)?;
    require!(
        health_factor >= config.min_health_factor,
        CustomError::BelowMinimumHealthFactor
    );
    Ok(())
}
pub fn calculate_health_factor(
    collateral: &Account<Collateral>,
    config: &Account<Config>,
    price_feed: &Account<PriceUpdateV2>,
) -> Result<u64> {
    let collateral_value_in_usdc = get_usd_value(&collateral.lamport_balance, price_feed)?;
    let collateral_adjusted_for_liquidation_threshold =
        (collateral_value_in_usdc * config.liquidation_threshold) / 100;
    if collateral.amount_minted == 0 {
        msg!("Health Factor Max");
        return Ok(u64::MAX);
    }

    // Calculate the health factor
    // Ratio of (adjusted collateral value) / (amount stablecoins minted)
    // Example: 500_000_000 / 500_000_000 = 1
    let health_factor = (collateral_adjusted_for_liquidation_threshold) / collateral.amount_minted;

    msg!("Health Factor : {}", health_factor);
    Ok(health_factor)
}
pub fn get_usd_value(amount_in_lamports: &u64, price_feed: &Account<PriceUpdateV2>) -> Result<u64> {
    let feed_id = get_feed_id_from_hex(FEED_ID)?;
    let price = price_feed.get_price_no_older_than(&Clock::get()?, MAXIMUM_AGE, &feed_id)?;
    // Check price is positive
    require!(price.price > 0, CustomError::InvalidPrice);

    // Adjust price to match lamports precision (9 decimals)
    // Example: Assuming 1 SOL = $2.00
    // price.price = 200_000_000 (from Pyth, 8 decimals)
    // price_in_usd = 200_000_000 * 10 = 2_000_000_000 (9 decimals)
    let price_in_usd = price.price as u128 * PRICE_FEED_DECIMAL_ADJUSTMENT;

    // Calculate USD value
    // Example: Convert 0.5 SOL to USD when 1 SOL = $2.00
    // amount_in_lamports = 500_000_000 (0.5 SOL)
    // price_in_usd = 2_000_000_000 (as calculated above)
    // LAMPORTS_PER_SOL = 1_000_000_000
    // amount_in_usd = (500_000_000 * 2_000_000_000) / 1_000_000_000 = 1_000_000_000 ($1.00)
    let amount_in_usd = (*amount_in_lamports as u128 * price_in_usd) / (LAMPORTS_PER_SOL as u128);
    Ok(amount_in_usd as u64)
}

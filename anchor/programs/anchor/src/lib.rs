use anchor_agnostic_orderbook::aob::state::MarketState;
use anchor_agnostic_orderbook::program::AnchorAgnosticOrderbook;
use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;



pub mod error;
pub mod state;
pub mod utils;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod anchor {
    use super::*;

    pub fn create_market(
        _ctx: Context<CreateMarket>,
        _signer_nonce: u64,
        _min_base_order_size: u64,
        _tick_size: u64,
        _cranker_reward: u64,
        _fee_tier_thresholds: [u64; 6],
        _fee_tier_taker_bps_rates: [u64; 7],
        _fee_tier_maker_bps_rebates: [u64; 7],
    ) -> Result<()> {
        Ok(())
    }
}

// TODO fix hardcoded space
#[derive(Accounts)]
pub struct CreateMarket<'info> {
    // has_one enforces `market.orderbook.pubkey (expected) = orderbook.pubkey (actual)`
    /// CHECK:
    #[account(init, payer = payer, space = 10240)]
    pub market: AccountInfo<'info>,
    #[account(init, payer = payer, space = 10240)]
    pub orderbook: AccountLoader<'info, MarketState>,
    #[account(mut)]
    pub base_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub quote_vault: Account<'info, TokenAccount>,
    pub market_admin: Signer<'info>,
    /// CHECK:
    #[account(init, payer = payer, space = 10240)]
    pub event_queue: AccountInfo<'info>,
    /// CHECK:
    #[account(init, payer = payer, space = 10240)]
    pub asks: AccountInfo<'info>,
    /// CHECK:
    #[account(init, payer = payer, space = 10240)]
    pub bids: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub aob_program: Program<'info, AnchorAgnosticOrderbook>,
}

#[derive(Accounts)]
pub struct NewOrder {}

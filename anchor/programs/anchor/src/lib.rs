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
        ctx: Context<CreateMarket>,
        signer_nonce: u64,
        min_base_order_size: u64,
        tick_size: u64,
        cranker_reward: u64,
        fee_tier_thresholds: [u64; 6],
        fee_tier_taker_bps_rates: [u64; 7],
        fee_tier_maker_bps_rebates: [u64; 7],
    ) -> Result<()> {
        anchor_agnostic_orderbook::cpi::create_market(
            CpiContext::new(
                AnchorAgnosticOrderbook,
                anchor_agnostic_orderbook::CreateMarket {
                    market: ctx.accounts.orderbook.to_account_info(),
                    event_queue: ctx.accounts.event_queue.to_account_info(),
                    bids: ctx.accounts.bids.to_account_info(),
                    asks: ctx.accounts.asks.to_account_info(),
                },
            ),
        );

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

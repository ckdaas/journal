#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("AsjZ3kWAUSQRNt2pZVeJkywhZ6gpLpHZmJjduPmKZDZZ");

#[program]
pub mod journal {
    use super::*;
}

#[account]
pub struct JournalEntryState {
    pub owner: Pubkey,
    pub title: String,
    pub message: String,
}

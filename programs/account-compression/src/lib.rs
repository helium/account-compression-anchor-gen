use std::str::FromStr;

use anchor_lang::prelude::*;

declare_id!("cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK");

declare_program!(account_compression);

#[derive(Clone)]
pub struct Noop {}

impl Id for Noop {
    fn id() -> Pubkey {
        Pubkey::from_str("noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV").unwrap()
    }
}

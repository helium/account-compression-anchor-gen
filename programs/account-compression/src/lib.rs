use std::str::FromStr;

use anchor_lang::prelude::*;

anchor_gen::generate_cpi_crate!("./idl.json");

declare_id!("cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK");

#[derive(Clone)]
pub struct Noop {}

impl Id for Noop {
    fn id() -> Pubkey {
        Pubkey::from_str("noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV").unwrap()
    }
}
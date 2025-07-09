#![allow(unexpected_cfgs)]
#![allow(deprecated)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("8KiiqftKSSHTE1zF1XmtcWf1zvppaFf9C7z4mmA46p3H");

#[program]
pub mod anchor_escrow {
    use super::*;

}

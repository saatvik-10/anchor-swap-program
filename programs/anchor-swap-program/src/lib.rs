pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("qyfaz8Aqt7cDGMor7JJxagUPJybZCUzPAqRGwFb3YXc");

#[program]
pub mod anchor_swap_program {

    use super::*;

    pub fn make_offer(ctx: Context<MakeOffer>) -> Result<()> {
        instruction::make_offer::send_offered_tokens_to_vault()?;

        instruction::make_offer::save_offer(ctx)
    }
}

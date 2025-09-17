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

    pub fn make_offer(
        ctx: Context<MakeOffer>,
        id: u64,
        token_a_offered_amount: u64,
        token_b_wanted_amount: u64,
    ) -> Result<()> {
        // Transfer tokens to vault
        transfer_token(
            &ctx.accounts.maker_token_account_a,
            &ctx.accounts.vault,
            &token_a_offered_amount,
            &ctx.accounts.token_mint_a,
            &ctx.accounts.maker,
            &ctx.accounts.token_program,
        )?;

        // Save offer data
        ctx.accounts.offer.set_inner(Offer {
            id,
            maker: ctx.accounts.maker.key(),
            token_mint_a: ctx.accounts.token_mint_a.key(),
            token_mint_b: ctx.accounts.token_mint_b.key(),
            token_b_wanted_amount,
            bump: ctx.bumps.offer,
        });

        Ok(())
    }
}

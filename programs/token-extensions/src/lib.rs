use anchor_lang::prelude::*;

pub mod instructions;
pub mod utils;
pub use instructions::*;
pub use utils::*;

declare_id!("542wLkW2AYsMqcPBMHgNixoMSenw88oRtAaZPaoFEmYe"); // Replace with your program ID

#[program]
pub mod token_extensions {
    use super::*;

    pub fn create_mint_account(
        ctx: Context<CreateMintAccount>,
        args: CreateMintAccountArgs,
    ) -> Result<()> {
        instructions::handler(ctx, args)
    }

    pub fn check_mint_extensions_constraints(
        _ctx: Context<CheckMintExtensionConstraints>,
    ) -> Result<()> {
        Ok(())
    }
}
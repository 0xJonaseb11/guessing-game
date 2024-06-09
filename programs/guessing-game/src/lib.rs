use anchor_lang::prelude::*;

use orao_solana_vrf::{
    cpi::accounts::Request, program::OraoVrf, state::NetworkState, CONFIG_ACCOUNT_SEED,
    RANDOMNESS_ACCOUNT_SEED,
};

declare_id!("HMDRWmYvL2A9xVKZG8iA1ozxi4gMKiHQz9mFkURKrG4"); // ! UPDATE ME

#[program]

pub mod guessing_game {
    use super::*;

    pub fn initialize(ctx: Context<GuessingGame>, force_seed: [u8; 32]) -> Result<()> {
        orao_solana_vrf::cpi::request(ctx.accounts.request_ctx(), force_seed)?;



        Ok(())
    }


// #[derive(Accounts)]
// #[instruction(user_guess: u64, force_seed: [u8; 32])] // new
}

#[derive(Accounts)]
#[instruction(force_seed: [u8; 32])]
pub struct GuessingGame<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [CONFIG_ACCOUNT_SEED.as_ref()],
        bump,
        seeds::program = orao_solana_vrf::ID
    )]
    pub network_state: Account<'info, NetworkState>,

    /// CHECK:
    #[account(mut)]
    pub treasury: AccountInfo<'info>,

    /// CHECK:
    #[account(
        mut,
        seeds = [RANDOMNESS_ACCOUNT_SEED.as_ref(), &force_seed],
        bump,
        seeds::program = orao_solana_vrf::ID
    )]
    pub random: AccountInfo<'info>,

    pub orao_vrf: Program<'info, OraoVrf>,
    pub system_program: Program<'info, System>,
}

impl<'info> GuessingGame<'info> {
    pub fn request_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Request<'info>> {
        let cpi_program = self.orao_vrf.to_account_info();
        let cpi_accounts = Request {
            payer: self.payer.to_account_info(), // ! because he is paying for thetx
            network_state: self.network_state.to_account_info(),
            treasury: self.treasury.to_account_info(),
            request: self.random.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };

        CpiContext::new(cpi_program, cpi_accounts)
    }
}
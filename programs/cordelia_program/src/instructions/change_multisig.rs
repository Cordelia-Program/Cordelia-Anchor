use anchor_lang::prelude::*;
use crate::{state::MultiSig, Errors, ChangeType};

#[derive(Accounts)]
pub struct ChangeMultisig<'info> {
    #[account(
        mut,
        seeds = [
            b"multi_sig",
            multi_sig.creator.as_ref(),
            multi_sig.name.as_bytes()
        ],
        bump = multi_sig.multisig_bump
    )]
    pub multi_sig: Account<'info, MultiSig>,

    #[account(
        seeds = [
            b"authority",
            multi_sig.key().as_ref()
        ],
        bump = multi_sig.authority_bump,
        signer
    )]
    pub authority: AccountInfo<'info>,
}

pub fn change_multisig_handler(
    ctx: Context<ChangeMultisig>, 
    change_type: ChangeType, 
    stratum: u8
) -> Result<()> {
    let multi_sig = &mut ctx.accounts.multi_sig;
    let s_index = stratum as usize;
    let strata_len = multi_sig.strata.len();

    require_gt!(strata_len, s_index, Errors::InvalidStratumNumber);

    match change_type {
        ChangeType::RemoveOwner { owner } => multi_sig.remove_owner(owner, s_index)?,
        ChangeType::ChangeM { new_m } => multi_sig.change_m(new_m, s_index)?,
        ChangeType::ActivateStratum => multi_sig.activate_stratum(s_index)?,
        ChangeType::DeactivateStratum => multi_sig.deactivate_stratum(s_index)?
    }

    multi_sig.version += 1;

    Ok(())
}
use anchor_lang::prelude::*;
use crate::{state::{MultiSig, ChangeReallocType},Errors};

#[derive(Accounts)]
#[instruction(change_type: ChangeReallocType)]
pub struct ChangeMultisigRealloc<'info> {
    #[account(
        mut,
        realloc = multi_sig.realloc_bytes(&change_type),
        realloc::payer = payer,
        realloc::zero = false,
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

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>
}

pub fn change_multisig_realloc_handler(
    ctx: Context<ChangeMultisigRealloc>, 
    change_type: ChangeReallocType
) -> Result<()> {
    let multi_sig = &mut ctx.accounts.multi_sig;

    match change_type {
        ChangeReallocType::AddOwner { owner, stratum } => {
            let s_index = stratum as usize;
            let strata_len = multi_sig.strata.len();

            require_gt!(strata_len, s_index, Errors::InvalidStratumNumber);
            multi_sig.add_owner(owner, stratum)?;
        },
        ChangeReallocType::AddStratum { stratum } => {
            multi_sig.add_stratum(stratum.owners, stratum.m)?;
        }
    }

    multi_sig.version += 1;

    ctx.accounts.multi_sig.reload()?;

    Ok(())
}
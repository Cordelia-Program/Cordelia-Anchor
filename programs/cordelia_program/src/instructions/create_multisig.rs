use anchor_lang::prelude::*;
use crate::{state::{MultiSig, Stratum},Errors,ID};

#[derive(Accounts)]
#[instruction(strata: Vec<Stratum>, name: String)]
pub struct CreateMultisig<'info> {
    #[account(
        init,
        payer=signer,
        space = MultiSig::len(strata, &name),
        seeds = [
            b"multi_sig",
            signer.key().as_ref(),
            name.as_bytes()
        ],
        bump
    )]
    pub multi_sig: Account<'info, MultiSig>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>
}

pub fn create_multisig_handler(ctx: Context<CreateMultisig>, strata: Vec<Stratum>, name: String) -> Result<()> {
    let mut strata = strata;
    require_gte!(25, name.len(), Errors::InvalidNameLen);

    let strata_len = strata.len();   
    require_gt!(strata_len, 0, Errors::InvalidStrataLen);
    require_gte!(u8::MAX as usize, strata_len, Errors::InvalidStrataLen);

    for (s_index,stratum) in strata.iter().enumerate() {
        let owners = &stratum.owners;
        let owners_len = owners.len();

        require_gt!(owners.len(), 0, Errors::InvalidOwnersCount);
        require_gte!(1220, owners.len(), Errors::InvalidOwnersCount);

        if s_index == 0 {
            require_gt!(stratum.m as usize, 0, Errors::ThresholdZero);   
        }

        for c_index in s_index .. strata_len {
            for (current_num, &current_key) in owners.iter().enumerate() {
                if s_index == c_index {
                    for &key in owners[current_num + 1 .. owners_len].iter() {
                        require_keys_neq!(current_key, key, Errors::DuplicateOwner);
                    }
                } else {
                    for &key in strata[c_index].owners.iter() {
                        require_keys_neq!(current_key, key, Errors::DuplicateOwner);
                    }
                }
            }
        }

        require_gte!(owners.len(), stratum.m as usize, Errors::ThresholdExceeds);
    }

    for stratum in strata.iter_mut() {
        stratum.owners.sort();
    }

    let multi_sig = &mut ctx.accounts.multi_sig;
    let signer = &ctx.accounts.signer;

    let (_authority, authority_bump) = Pubkey::find_program_address(
        &[b"authority", multi_sig.key().as_ref()],
        &ID
    );

    **multi_sig = MultiSig::new(
        strata, 
        authority_bump, 
        name,
        signer.key(),
        *ctx.bumps.get("multi_sig").unwrap()
    );

    Ok(())
}
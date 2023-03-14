use anchor_lang::prelude::*;
use crate::{state::{MultiSig, Transaction},Errors};

#[derive(Accounts)]
#[instruction(owner_stratum: u8, description: String)]
pub struct CreateTransaction<'info> {
    #[account(
        init,
        payer = signer,
        space = Transaction::len(multi_sig.strata.len(), description),
        seeds = [
            b"transaction",
            multi_sig.key().as_ref(),
            &multi_sig.transaction_count.to_le_bytes()
        ],
        bump
    )]
    pub transaction: Account<'info, Transaction>,

    #[account(
        mut,
        seeds = [
            b"multi_sig",
            multi_sig.creator.as_ref(),
            multi_sig.name.as_bytes()
        ],
        bump = multi_sig.multisig_bump,
        constraint = multi_sig.strata.len() > owner_stratum as usize @ Errors::InvalidStratumNumber
    )]
    pub multi_sig: Account<'info, MultiSig>,

    #[account(
        mut,
        constraint = multi_sig.is_owner_stratum(signer.key(), owner_stratum as usize).is_some() @ Errors::InvalidOwner
    )]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>
}

pub fn create_transaction_handler(
    ctx: Context<CreateTransaction>, 
    _owner_stratum: u8,
    description: String
) -> Result<()> {
    let multi_sig = &mut ctx.accounts.multi_sig;
    let transaction = &mut ctx.accounts.transaction;
    let owner = &ctx.accounts.signer;

    require_gte!(140, description.len(), Errors::InvalidDescriptionLen);

    let transaction_num = multi_sig.transaction_count;
    let version = multi_sig.version;
    let stratum_count = multi_sig.strata.len();

    multi_sig.transaction_count = multi_sig.transaction_count.checked_add(1).unwrap();

    **transaction = Transaction::new(
        multi_sig.key(),
        owner.key(),
        transaction_num,
        *ctx.bumps.get("transaction").unwrap(),
        version,
        stratum_count,
        description
    );

    Ok(())
}
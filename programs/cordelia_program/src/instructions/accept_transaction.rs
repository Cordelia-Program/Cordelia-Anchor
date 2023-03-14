use anchor_lang::prelude::*;
use crate::{state::{MultiSig, Transaction, TransactionStatus},Errors};

#[derive(Accounts)]
#[instruction(owner_stratum: u8)]
pub struct AcceptTransaction<'info> {
    #[account(
        mut,
        seeds = [
            b"transaction",
            multi_sig.key().as_ref(),
            &transaction.transaction_num.to_le_bytes()
        ],
        bump = transaction.bump,
        constraint = transaction.multi_sig == multi_sig.key() @ Errors::InvalidTransaction,
        constraint = transaction.status == TransactionStatus::Vote @ Errors::NotVoteStatus,
        constraint = transaction.version == multi_sig.version @ Errors::VersionOutdated
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
    pub signer: Signer<'info>
}

pub fn accept_transaction_handler(
    ctx: Context<AcceptTransaction>, 
    _owner_stratum: u8,
) -> Result<()> {
    let transaction = &mut ctx.accounts.transaction;
    let multi_sig = &ctx.accounts.multi_sig;

    for (index, stratum) in multi_sig.strata.iter().enumerate() {
        if stratum.active {
            require_gte!(transaction.accepted[index], stratum.m, Errors::InsufficientVotes);
        }
    }

    transaction.status = TransactionStatus::Accepted;

    Ok(())
}
use anchor_lang::prelude::*;
use crate::{state::{MultiSig, Transaction, TransactionStatus, VoteRecord},Errors};

#[derive(Accounts)]
#[instruction(owner_stratum: u8)]
pub struct VoteTransaction<'info> {
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
        init,
        space = VoteRecord::LEN,
        payer = signer,
        seeds = [
            b"vote",
            transaction.key().as_ref(),
            signer.key().as_ref()
        ],
        bump,
    )]
    pub vote_record: Account<'info, VoteRecord>,

    #[account(
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

pub fn vote_transaction_handler(
    ctx: Context<VoteTransaction>, 
    owner_stratum: u8,
    is_accept: bool
) -> Result<()> {
    let transaction = &mut ctx.accounts.transaction;
    let vote_record = &mut ctx.accounts.vote_record;
    let multi_sig = &ctx.accounts.multi_sig;
    let signer = &ctx.accounts.signer;

    let s_index = owner_stratum as usize;

    let target_stratum_len = multi_sig.strata[s_index].owners.len() as u16;
    let target_stratum_m = multi_sig.strata[s_index].m;

    match is_accept {
        true => {
            transaction.accepted[s_index] += 1;
        },
        false => {
            transaction.rejected[s_index] += 1;

            let max_rejection = target_stratum_len - target_stratum_m;

            if transaction.rejected[s_index] > max_rejection {
                transaction.status = TransactionStatus::Rejected;
            }
        }
    }

    **vote_record = VoteRecord::new(transaction.key(), signer.key());

    Ok(())
}
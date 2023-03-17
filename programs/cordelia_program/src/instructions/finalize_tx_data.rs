use anchor_lang::prelude::*;
use crate::{state::{MultiSig, Transaction, TransactionStatus, TxData},Errors};

#[derive(Accounts)]
pub struct FinalizeTxData<'info> {
    #[account(
        mut,
        seeds = [
            b"transaction",
            multi_sig.key().as_ref(),
            &transaction.transaction_num.to_le_bytes()
        ],
        bump = transaction.bump,
        constraint = transaction.multi_sig == multi_sig.key() @ Errors::InvalidTransaction,
        constraint = transaction.status == TransactionStatus::Initiated @ Errors::AlreadyFinalized,
        constraint = transaction.version == multi_sig.version @ Errors::VersionOutdated,
        address = tx_data.transaction @ Errors::InvalidTxData
    )]
    pub transaction: Account<'info, Transaction>,

    #[account(
        seeds = [
            b"data",
            transaction.key().as_ref(),
        ],
        bump = tx_data.bump,
    )]
    pub tx_data: Account<'info, TxData>,

    #[account(
        seeds = [
            b"multi_sig",
            multi_sig.creator.as_ref(),
            multi_sig.name.as_bytes()
        ],
        bump = multi_sig.multisig_bump,
    )]
    pub multi_sig: Account<'info, MultiSig>,

    #[account(
        address = transaction.owner @ Errors::InvalidCreator
    )]
    pub signer: Signer<'info>
}

pub fn finalize_tx_data_handler(ctx: Context<FinalizeTxData>) -> Result<()> {
    let transaction = &mut ctx.accounts.transaction;
    transaction.status = TransactionStatus::Vote;
    Ok(())
}
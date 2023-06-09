use anchor_lang::prelude::*;
use crate::{state::{MultiSig, Transaction, TransactionStatus, InstructionData, TxData, TransactionType},Errors};

#[derive(Accounts)]
#[instruction(instructions: Vec<InstructionData>)]
pub struct CreateTxData<'info> {
    #[account(
        seeds = [
            b"transaction",
            multi_sig.key().as_ref(),
            &transaction.transaction_num.to_le_bytes()
        ],
        bump = transaction.bump,
        constraint = transaction.multi_sig == multi_sig.key() @ Errors::InvalidTransaction,
        constraint = transaction.status == TransactionStatus::Initiated @ Errors::AlreadyFinalized,
        constraint = transaction.version == multi_sig.version @ Errors::VersionOutdated
    )]
    pub transaction: Account<'info, Transaction>,

    #[account(
        init,
        space = TxData::len(TransactionType::Legacy { data: instructions}),
        payer = signer,
        seeds = [
            b"data",
            transaction.key().as_ref(),
        ],
        bump,
    )]
    pub tx_data: Account<'info, TxData>,

    #[account(
        seeds = [
            b"multi_sig",
            multi_sig.creator.as_ref(),
            multi_sig.name.as_bytes()
        ],
        bump = multi_sig.multisig_bump
    )]
    pub multi_sig: Account<'info, MultiSig>,

    #[account(
        mut,
        address = transaction.owner @ Errors::InvalidCreator
    )]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>
}

pub fn create_tx_data_handler(
    ctx: Context<CreateTxData>, 
    instructions: Vec<InstructionData>
) -> Result<()> {
    let transaction = &ctx.accounts.transaction;
    let tx_data = &mut ctx.accounts.tx_data;

    **tx_data = TxData::new(
        transaction.key(), 
        instructions, 
        *ctx.bumps.get("tx_data").unwrap(),
        None
    );

    Ok(())
}
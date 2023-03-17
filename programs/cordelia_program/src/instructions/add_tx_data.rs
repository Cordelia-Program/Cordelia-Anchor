use anchor_lang::prelude::*;
use crate::{state::{MultiSig, Transaction, TransactionStatus, TxData, InstructionAccount},Errors};

#[derive(Accounts)]
#[instruction(keys: Vec<InstructionAccount>)]
pub struct AddTxData<'info> {
    #[account(
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
        mut,
        realloc = tx_data.realloc_bytes(&keys),
        realloc::payer = signer,
        realloc::zero = false,
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

pub fn add_tx_data_handler(
    ctx: Context<AddTxData>, 
    keys: Vec<InstructionAccount>,
    instruction_index: u8
) -> Result<()> {
    let tx_data = &mut ctx.accounts.tx_data;
    let i_index = instruction_index as usize;

    require_gte!(tx_data.instructions.len(), i_index, Errors::InvalidTxDataIndex);
    tx_data.instructions[i_index].keys.extend_from_slice(&keys);
    
    Ok(())
}
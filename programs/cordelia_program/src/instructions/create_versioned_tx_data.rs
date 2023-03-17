use anchor_lang::prelude::*;
use crate::{state::{MultiSig, Transaction, TransactionStatus, 
    InstructionData, TxData, InstructionAccount, 
    TransactionType, VersionedInstructionData
},
    Errors
};

#[derive(Accounts)]
#[instruction(versioned_ixs: Vec<VersionedInstructionData>)]
pub struct CreateVersionedTxData<'info> {
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
        space = TxData::len(TransactionType::Versioned {data : versioned_ixs }),
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
        address = transaction.owner @ Errors::InvalidOwner
    )]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>
}

pub fn create_versioned_tx_data_handler(
    ctx: Context<CreateVersionedTxData>, 
    versioned_ixs: Vec<VersionedInstructionData>,
    lookup_table: Option<Pubkey>
) -> Result<()> {
    let transaction = &ctx.accounts.transaction;
    let tx_data = &mut ctx.accounts.tx_data;

    let remaining_accounts = &ctx.remaining_accounts;

    let instructions: Vec<InstructionData> = versioned_ixs.into_iter().map(|f| {
        InstructionData {
            program_id: remaining_accounts[f.program_id_index as usize].key(),
            data: f.data,
            keys: f.keys.into_iter().map(|k| {
                InstructionAccount {
                    pubkey: remaining_accounts[k.pubkey_index as usize].key(),
                    is_signer: k.is_signer,
                    is_writable: k.is_writable
                }
            }).collect()
        }
    }).collect();

    **tx_data = TxData::new(
        transaction.key(), 
        instructions, 
        *ctx.bumps.get("tx_data").unwrap(),
        lookup_table
    );

    Ok(())
}
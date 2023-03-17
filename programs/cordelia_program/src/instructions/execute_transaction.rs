use anchor_lang::{prelude::*, solana_program::{instruction::Instruction,program::invoke_signed}};
use crate::{state::{MultiSig, Transaction, TransactionStatus, TxData},Errors, ID};

#[derive(Accounts)]
#[instruction(owner_stratum: u8)]
pub struct ExecuteTransaction<'info> {
    #[account(
        mut,
        seeds = [
            b"transaction",
            multi_sig.key().as_ref(),
            &transaction.transaction_num.to_le_bytes()
        ],
        bump = transaction.bump,
        constraint = transaction.multi_sig == multi_sig.key() @ Errors::InvalidTransaction,
        constraint = transaction.status == TransactionStatus::Accepted @ Errors::NotAccepted,
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
        constraint = multi_sig.strata.len() > owner_stratum as usize @ Errors::InvalidStratumNumber
    )]
    pub multi_sig: Account<'info, MultiSig>,

    #[account(
        mut,
        constraint = multi_sig.is_owner_stratum(signer.key(), owner_stratum as usize).is_some() @ Errors::InvalidOwner
    )]
    pub signer: Signer<'info>,
}

pub fn execute_transaction_handler<'info>(
    ctx: Context<'_, '_, '_, 'info, ExecuteTransaction<'info>>, 
    _owner_stratum: u8
) -> Result<()> {
    let multi_sig = &ctx.accounts.multi_sig;
    let tx_data = &ctx.accounts.tx_data;
    let transaction = &mut ctx.accounts.transaction;
    let multi_sig_key = multi_sig.key();

    let authority_seeds = [
        b"authority",
        multi_sig_key.as_ref(),
        &[multi_sig.authority_bump],
    ];

    let remaining_accounts = &mut ctx.remaining_accounts.iter();

    for ix_data in tx_data.instructions.iter() {
        let ix_data = ix_data.clone();

        let program_id = next_account_info(remaining_accounts)?;

        require_keys_eq!(program_id.key(), ix_data.program_id, Errors::InvalidInstructionAccount);

        let ix_keys = ix_data.keys.clone();
        let mut ix: Instruction = Instruction::from(ix_data);
        let mut ix_accounts = vec![program_id.clone()];

        for key in &ix_keys {
            let ix_account = next_account_info(remaining_accounts)?;
            require_keys_eq!(key.pubkey, ix_account.key(), Errors::InvalidInstructionAccount);

            ix_accounts.push(ix_account.clone());
        }

        if program_id.key() == ID {
            // If the internal instruction is execute_transaction(), return error
            if ix.data.get(0..8) == Some(vec![0xe7, 0xad, 0x31, 0x5b, 0xeb, 0x18, 0x44, 0x13].as_slice()) {
                return Err(Errors::InvalidInstruction.into());
            }
            
            // if the internal instruction is change_multisig_realloc(), change the payer
            if ix.data.get(0..8) == Some(vec![0x98, 0x5a, 0x30, 0x68, 0x6b, 0x7a, 0x0b, 0x71].as_slice()) {
                ix_accounts[3] = ctx.accounts.signer.to_account_info();
                ix.accounts[2] = AccountMeta::new(ctx.accounts.signer.key(), true);
            }
        }

        invoke_signed(&ix,&ix_accounts,&[&authority_seeds])?;
    }

    transaction.status = TransactionStatus::Executed;
    ctx.accounts.multi_sig.reload()?;
    Ok(())
}
use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;

use instructions::*;
use state::{Stratum, InstructionData, ChangeReallocType, ChangeType, VersionedInstructionData, InstructionAccount};

declare_id!("2h4ZQdRQESZETBWh6mC9pgLGca8ytMR5GcCqMsUACyyY");

#[program]
pub mod cordelia_program {
    use super::*;

    pub fn create_multisig(ctx: Context<CreateMultisig>, strata: Vec<Stratum>, name: String) -> Result<()> {
        create_multisig_handler(ctx, strata, name)
    }

    pub fn create_transaction(
        ctx: Context<CreateTransaction>, 
        owner_stratum: u8,
        description: String
    ) -> Result<()> {
        create_transaction_handler(ctx, owner_stratum, description)
    }

    pub fn create_tx_data(
        ctx: Context<CreateTxData>, 
        instructions: Vec<InstructionData>
    ) -> Result<()> {
        create_tx_data_handler(ctx, instructions)
    }

    pub fn create_versioned_tx_data(
        ctx: Context<CreateVersionedTxData>, 
        versioned_ixs: Vec<VersionedInstructionData>,
        lookup_table: Option<Pubkey>
    ) -> Result<()> {
        create_versioned_tx_data_handler(ctx, versioned_ixs, lookup_table)
    }

    pub fn add_tx_data(
        ctx: Context<AddTxData>, 
        keys: Vec<InstructionAccount>,
        instruction_index: u8
    ) -> Result<()> {
        add_tx_data_handler(ctx, keys, instruction_index)
    }

    pub fn finalize_tx_data(ctx: Context<FinalizeTxData>) -> Result<()> {
        finalize_tx_data_handler(ctx)
    }

    pub fn vote_transaction(
        ctx: Context<VoteTransaction>, 
        owner_stratum: u8,
        is_accept: bool
    ) -> Result<()> {
        vote_transaction_handler(ctx, owner_stratum, is_accept)
    }

    pub fn accept_transaction(
        ctx: Context<AcceptTransaction>, 
        owner_stratum: u8,
    ) -> Result<()> {
        accept_transaction_handler(ctx, owner_stratum)
    }

    pub fn execute_transaction<'info> (
        ctx: Context<'_, '_, '_, 'info, ExecuteTransaction<'info>>, 
        owner_stratum: u8
    ) -> Result<()> {
        execute_transaction_handler(ctx, owner_stratum)
    }

    pub fn change_multisig(
        ctx: Context<ChangeMultisig>, 
        change_type: ChangeType, 
        stratum: u8
    ) -> Result<()> {
        change_multisig_handler(ctx, change_type, stratum)
    }

    pub fn change_multisig_realloc(
        ctx: Context<ChangeMultisigRealloc>, 
        change_type: ChangeReallocType
    ) -> Result<()> {
        change_multisig_realloc_handler(ctx, change_type)
    }

}

#[error_code]
pub enum Errors {
    #[msg("Invalid number of stratum provided")]
    InvalidStrataLen,

    #[msg("Invalid number of owners provided in a stratum")]
    InvalidOwnersCount,

    #[msg("Threshold can't exceed the number of owners")]
    ThresholdExceeds,

    #[msg("Threshold of the first stratum can't be zero")]
    ThresholdZero,

    #[msg("Threshold of the stratum must be zero before deactivation")]
    ThresholdNotZero,

    #[msg("The signer is not the owner of the multi-sig")]
    InvalidOwner,

    #[msg("The signer is not the creator of this transaction")]
    InvalidCreator,

    #[msg("Duplicate owner provided in the stratum")]
    DuplicateOwner,

    #[msg("Owner already exists in this multi-sig")]
    OwnerAlreadyExists,

    #[msg("Owner doesn't exist in this multi-sig")]
    OwnerDoesntExist,

    #[msg("Transaction provided is not from this multisig")]
    InvalidTransaction,

    #[msg("The account doesn't match with the instruction data")]
    InvalidInstructionAccount,

    #[msg("The instruction is not permitted in multisig")]
    InvalidInstruction,

    #[msg("The stratum is inactive")]
    InactiveStratum,

    #[msg("The stratum is already active")]
    ActiveStratum,

    #[msg("The first stratum cannot be deactivated")]
    CannotDeactivateFirst,

    #[msg("The stratum provided doesn't exist")]
    InvalidStratumNumber,

    #[msg("The transaction is not open for voting")]
    NotVoteStatus,

    #[msg("The version of the transaction is out-dated")]
    VersionOutdated,

    #[msg("Insufficient votes to accept the transaction")]
    InsufficientVotes,

    #[msg("The transaction data is already finalized")]
    AlreadyFinalized,

    #[msg("The transaction is not accepted by the owners")]
    NotAccepted,

    #[msg("Tx Data doesn't belong to the given transaction")]
    InvalidTxData,

    #[msg("Transaction description is too long")]
    InvalidDescriptionLen,

    #[msg("Multisig name is too long")]
    InvalidNameLen,
    
    #[msg("Tx Data doesn't have the provided element")]
    InvalidTxDataIndex
}
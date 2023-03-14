use anchor_lang::prelude::*;

#[account]
pub struct Transaction {
    /// The address of the multi-sig wallet (32)
    pub multi_sig: Pubkey,

    /// The address of the owner who created the tx (32)
    pub owner: Pubkey,

    /// The transaction number in the multi-sig (4)
    pub transaction_num: u32,

    /// The version of the transaction (4)
    pub version: u32,

    /// Transaction Status (1)
    pub status: TransactionStatus,

    /// The bump of the transaction account's PDA (1)
    pub bump: u8,

    /// The number of owners accepted the tx
    pub accepted: Vec<u16>,

    /// The number of owners rejected the tx
    pub rejected: Vec<u16>,

    /// The description of the transaction (max 140 + 4)
    pub description: String
}

impl Transaction {
    pub fn len(stratum_count: usize, description: String) -> usize {
        8 + 32 + 32 + 4 + 4 + 1 + 1 + 4 + stratum_count * 2 + 4 + stratum_count * 2 + 4 + description.len()
    }

    pub fn new(
        multi_sig: Pubkey,
        owner: Pubkey,
        transaction_num: u32,
        bump: u8,
        version: u32,
        stratum_count: usize,
        description: String
    ) -> Self {
        let v: Vec<u16> = vec![0; stratum_count];

        Self {
            multi_sig,
            owner,
            transaction_num,
            bump,
            version,
            status: TransactionStatus::Initiated,
            accepted: v.clone(),
            rejected: v.clone(),
            description
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Eq, PartialEq)]
pub enum TransactionStatus {
    Initiated,
    Vote,
    Accepted,
    Rejected,
    Executed
}


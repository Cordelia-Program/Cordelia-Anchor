use anchor_lang::prelude::*;

#[account]
pub struct VoteRecord {
    pub transaction: Pubkey,
    pub owner: Pubkey
}

impl VoteRecord {
    pub const LEN: usize = 8 + 32 + 32;

    pub fn new(transaction: Pubkey, owner: Pubkey) -> Self {

        Self {
            transaction,
            owner
        }
    }
}

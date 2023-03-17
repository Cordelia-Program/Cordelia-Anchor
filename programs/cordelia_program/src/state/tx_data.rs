use anchor_lang::{prelude::*, solana_program::instruction::Instruction};

#[account]
pub struct TxData {
    pub transaction: Pubkey,
    pub instructions: Vec<InstructionData>,
    pub bump: u8,
    pub lookup_table: Option<Pubkey>
}

impl TxData {
    pub fn len(data_type: TransactionType) -> usize {
        let mut size = 8 + 32 + 1 + 4 + 33;

        match data_type {
            TransactionType::Legacy { data } => {
                for ix in data {
                    size += 32 + 4 + ix.data.len() + 4 + ix.keys.len() * (32 + 1 + 1); 
                }
            },
            TransactionType::Versioned { data } => {
                for ix in data {
                    size += 32 + 4 + ix.data.len() + 4 + ix.keys.len() * (32 + 1 + 1); 
                }
            }
        }

        size
    }

    pub fn realloc_bytes(&self, new_keys: &Vec<InstructionAccount>) -> usize {
        let current_len = TxData::len(TransactionType::Legacy { data: self.instructions.clone() });

        let require_bytes = new_keys.len() * (32 + 1 + 1);
        current_len + require_bytes
    }
    
    pub fn new(
        transaction: Pubkey, 
        instructions: Vec<InstructionData>,
        bump: u8,
        lookup_table: Option<Pubkey>
    ) -> Self {

        Self {
            transaction,
            instructions,
            bump,
            lookup_table
        }
    }
}

#[derive(AnchorDeserialize,AnchorSerialize, Clone)]
pub struct InstructionData {
    pub program_id: Pubkey,
    pub data: Vec<u8>,
    pub keys: Vec<InstructionAccount>
}

#[derive(AnchorDeserialize,AnchorSerialize, Clone, )]
pub struct InstructionAccount {
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool
}

#[derive(AnchorDeserialize,AnchorSerialize, Clone)]
pub struct VersionedInstructionData {
    pub program_id_index: u8,
    pub data: Vec<u8>,
    pub keys: Vec<VersionedInstructionAccount>
}

#[derive(AnchorDeserialize,AnchorSerialize, Clone, )]
pub struct VersionedInstructionAccount {
    pub pubkey_index: u8,
    pub is_signer: bool,
    pub is_writable: bool
}

#[derive(AnchorDeserialize,AnchorSerialize, Clone, )]
pub enum TransactionType {
    Legacy { data : Vec<InstructionData>},
    Versioned { data: Vec<VersionedInstructionData>}
}

impl From<InstructionData> for Instruction {
    fn from(instruction: InstructionData) -> Self {
        Instruction { 
            program_id: instruction.program_id, 
            accounts: instruction.keys.iter().map(|account| AccountMeta {
                pubkey: account.pubkey,
                is_signer: account.is_signer,
                is_writable: account.is_writable
            }).collect(), 
            data: instruction.data
        }
    }
}
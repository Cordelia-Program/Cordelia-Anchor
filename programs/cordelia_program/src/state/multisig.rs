use anchor_lang::prelude::*;
use crate::Errors;

#[account]
pub struct MultiSig {
    pub strata: Vec<Stratum>,
    pub transaction_count: u32,
    pub authority_bump: u8,
    pub multisig_bump: u8,
    pub version: u32,
    pub name: String,
    pub creator: Pubkey,
    pub recovery: bool
}

impl MultiSig {
    pub fn len(strata: Vec<Stratum>, name: &String) -> usize {
        let mut size: usize = 8 + 4 + 4 + 4 + 1 + 1 + 32 + 1; //disc, vec, tx_count, creator, recovery and bumps
        
        for stratum in strata {
            size += 4 + stratum.owners.len() * 32 + 2 + 1;
        }

        size += 4 + name.len();

        size
    }

    pub fn new(
        strata: Vec<Stratum>,
        authority_bump: u8, 
        name: String, 
        creator: Pubkey,
        multisig_bump: u8
    ) -> Self {
        let mut strata = strata;

        for stratum in strata.iter_mut() {
            stratum.active = true;
        }

        Self { 
            strata,
            transaction_count: 0,
            authority_bump,
            multisig_bump,
            version: 0,
            name,
            creator,
            recovery: false
        }
    }

    pub fn realloc_bytes(&self, change_type: &ChangeReallocType) -> usize {
        let req_len = MultiSig::len(self.strata.clone(), &self.name);

        match change_type {
            ChangeReallocType::AddOwner { owner: _, stratum: _ } => req_len + 32,
            ChangeReallocType::AddStratum { stratum } => req_len + 4 + stratum.owners.len() * 32 + 2 + 1
        }
    }

    pub fn is_owner_stratum(&self, address: Pubkey, owner_stratum: usize) -> Option<usize> {
        match self.strata[owner_stratum].owners.binary_search(&address) {
            Ok(i) => Some(i),
            _ => None
        }
    }

    pub fn is_owner_wallet(&self, address: Pubkey) -> bool {
        for stratum in self.strata.iter() {
            let result = stratum.owners.binary_search(&address);

            match result {
                Ok(_i) => {
                    return true;
                },
                _ => {
                    continue;
                }
            }
        }

        false
    }

    pub fn add_owner(&mut self, owner: Pubkey, stratum: u8) -> Result<()> {
        if self.is_owner_wallet(owner) {
            return Err(Errors::OwnerAlreadyExists.into());
        }

        let target_stratum = &mut self.strata[stratum as usize];
        let target_owners = &mut target_stratum.owners;

        require!(target_stratum.active, Errors::InactiveStratum);
        require_gt!(1220, target_owners.len(), Errors::InvalidOwnersCount);

        target_owners.push(owner);
        target_owners.sort();
        Ok(())
    }

    pub fn remove_owner(&mut self, owner: Pubkey, s_index: usize) -> Result<()> {
        let owner_status = self.is_owner_stratum(owner, s_index);
        let target_stratum = &mut self.strata[s_index];

        match owner_status {
            Some(index) => {
                require!(target_stratum.active, Errors::InactiveStratum);
                require_gte!(target_stratum.owners.len() - 1, target_stratum.m as usize, Errors::ThresholdExceeds);
                target_stratum.owners.remove(index);
            }, 
            _ => {
                return Err(Errors::OwnerDoesntExist.into());
            }
        }
        
        Ok(())
    }

    pub fn add_stratum(&mut self, owners: Vec<Pubkey>, m: u16) -> Result<()> {
        let mut owners = owners;

        require_gt!(u8::MAX as usize, self.strata.len(), Errors::InvalidStrataLen);
        require_gt!(owners.len(), 0, Errors::InvalidOwnersCount);
        require_gte!(1220, owners.len(), Errors::InvalidOwnersCount);

        require_gte!(owners.len(), m as usize, Errors::ThresholdExceeds);

        for owner in owners.iter() {
            require!(!self.is_owner_wallet(*owner), Errors::OwnerAlreadyExists);
        }

        owners.sort();

        self.strata.push(Stratum {
            owners,
            m,
            active: true
        });

        Ok(())
    }

    pub fn deactivate_stratum(&mut self, s_index: usize) -> Result<()> {
        if s_index == 0 {
            return Err(Errors::CannotDeactivateFirst.into());
        }

        let target_stratum = &mut self.strata[s_index];

        require!(target_stratum.active, Errors::InactiveStratum);
        require_eq!(target_stratum.m, 0, Errors::ThresholdNotZero);

        target_stratum.active = false;
        target_stratum.owners = Vec::new();
        Ok(())
    }

    pub fn activate_stratum(&mut self, s_index: usize) -> Result<()> {
        require!(!self.strata[s_index].active, Errors::ActiveStratum);
        self.strata[s_index].active = true;

        Ok(())
    }

    pub fn change_m(&mut self, new_m: u16, s_index: usize) -> Result<()> {
        let target_stratum = &mut self.strata[s_index];

        require_gte!(target_stratum.owners.len(), new_m as usize, Errors::ThresholdExceeds);
        require!(target_stratum.active, Errors::InactiveStratum);

        if s_index == 0 && new_m == 0 {
            return Err(Errors::ThresholdZero.into());
        }

        target_stratum.m = new_m;

        Ok(())
    }
}

#[derive(AnchorDeserialize,AnchorSerialize, Clone)]
pub struct Stratum {
    pub owners: Vec<Pubkey>,
    pub m: u16,
    pub active: bool
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub enum ChangeReallocType {
    AddOwner {owner: Pubkey, stratum: u8},
    AddStratum {stratum: Stratum}
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub enum ChangeType {
    RemoveOwner { owner: Pubkey },
    ChangeM { new_m: u16 },
    ActivateStratum,
    DeactivateStratum
}
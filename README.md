> warning: The program is not properly tested yet. Kindly only use it for the testing purpose. Refrain from using the code for production purpose in any way.

# Cordelia Program

## Cordelia is an experimental Solana multi-sig program which lets you create multi-sig instances and execute transactions containing arbitrary instructions using CPI calls. We have taken inspiration from Squads MPL program.

Rather than a normal M-of-N owner scheme, Cordelia is created using Strata model. The owners of the multi-sig can be divided into different groups (known as Stratum) and threshold can be set separately for every stratum. To successfully execute any transaction, owners from every stratum have to vote in number atleast equal to stratum's threshold count.

Program Address: 2h4ZQdRQESZETBWh6mC9pgLGca8ytMR5GcCqMsUACyyY (deployed on both mainnet and devnet)

## Program State Model

- Multi-sig has `strata` field which is a vector of `Stratum`. The first element of `strata` vector is a base `Stratum` and must have at least 1 owner. Every `Stratum` can have separate owners and threshold and an individual `Pubkey` can only be a member of a single stratum. `multi_sig` account is seeded using the creator's key and a unique name chosen at the time of creation (can be of maximum 25 characters).

- Transactions are seeded using `transaction_count` field saved in `multi_sig` which is incremented after every transaction creation. `transaction` has two vector fields: `accepted` and `rejected` whose length is equal to the length of `strata` field in `multi_sig` and save the number of accepted or rejected votes for a particular transaction in the form of `u16`. Transaction has a `version` field and becomes outdated if any change is made to `strata` after its creation if it is still pending acceptance.

- TxData is an account seeded with the `Pubkey` of `transaction` and saves the instructions in the form of vector. Instructions are saved in a custom format known as `InstructionData` which further has `keys` field which saves `AccountMeta` needed for instruction in a custom type: `InstructionAccount`.

- Vote Record is a PDA seeded with the `Pubkey` of transaction and owner. It ensures that an owner can only vote once for each transaction.

## Program Instruction Model

- Multi-sig can be created using `create_multisig` instruction by passing the list of owners and threshold as vector of `Stratum`.

- A transaction can be added to multi-sig using `create_transaction` instruction and increments the field in `transaction_count` in `multi_sig`.

- TxData for a transaction can be created using either of the two instructions: `create_tx_data` for legacy transactions and `create_versioned_tx_data` for Version transactions. In `create_tx_data` instruction, accounts needed for the instruction are passed as an argument whereas in `create_versioned_tx_data`, accounts are passed in `remaining_accounts` array in a specified format.

- More accounts can be inserted into instructions saved in TxData account by using `add_tx_data` which accepts two arguments - vector of `InstructionAccount` and index of the instuction where accounts are to be inserted. The data in TxData can be finalized and transaction can be sent to vote using `finalize_tx_data` instruction.

- `vote_transaction` instruction is used to vote for a transaction and the vote is expressed in the form of `bool` argument where `true` means accept and `false` means reject. If the number of rejected votes become more such that it is no longer possible to reach the threshold, the transaction's status is set to `Rejected` and no further interaction with the transaction is allowed. 

- `accept_transaction` instruction is used to mark the transaction's status as `Accepted` if the transaction satisfies threshold requirement of every Stratum. The transaction can only be executed if the status is set to `Accepted`.

- The transaction can finally be executed using `execute_transaction` which signs the transaction using PDA authority seeded with `multi_sig` key and `b"authority"`.

- To make the internal changes, two special instructions are used: `change_multi_sig` and `change_multi_sig_realloc`. Both are used for different purposes as follows:

Change Multi Sig: To make changes which doesn't require a change in the size of `multi_sig` account. It accepts the type of change in the form of enum which can have four values: RemoveOwner, ChangeM (for threshold), ActivateStratum and DeactivateStratum.

Change Multi Sig Realloc: To make changes which requires a change in the size of `multi_sig` account. It accepts the type of change in the form of enum which can have two values: AddOwner and AddStratum. 

## Constraints

At this moment the constraints are set manually to avoid overflow but will be set programmatically after the proper testing:

- Max number of Stratums: 256
- Max number of owners in Stratum: 1220
- Max number of transactions in Multi-sig: u32::MAX

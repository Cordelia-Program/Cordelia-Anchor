import * as anchor from "@project-serum/anchor";
import { Program, web3, utils, BN } from "@project-serum/anchor";

import { CordeliaProgram } from "../target/types/cordelia_program";

describe("cordelia_program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CordeliaProgram as Program<CordeliaProgram>;

  const multiSigName = "Test DAO";

  const [multiSig] = web3.PublicKey.findProgramAddressSync([
    utils.bytes.utf8.encode("multi_sig"),
    program.provider.publicKey.toBytes(),
    utils.bytes.utf8.encode(multiSigName),
  ],program.programId);

  const [multisigAuthority] = web3.PublicKey.findProgramAddressSync([
    utils.bytes.utf8.encode("authority"),
    multiSig.toBytes(),
  ],program.programId);

  async function getTransactionKey(create: boolean) {
    const multisigData = await program.account.multiSig.fetch(multiSig);
    const count = create ? multisigData.transactionCount as number : multisigData.transactionCount as number - 1;

    const [transactionPda] = web3.PublicKey.findProgramAddressSync([
        utils.bytes.utf8.encode("transaction"),
        multiSig.toBytes(),
        new BN(count).toBuffer("le", 4)
    ],program.programId);

    return transactionPda;
  }

  it("It creates multi-sig!", async () => {
    const tx = await program.methods.createMultisig([
      {
          owners: [program.provider.publicKey],
          m: 1,
          active: false
      },    
    ],multiSigName)
    .accounts({
      multiSig
    })
    .rpc();

    console.log("Your transaction signature", tx);
  });


  it("It creates transaction!", async () => {
    const transaction = await getTransactionKey(true);

    const tx = await program.methods.createTransaction(
      0,
      "Add a new owner to the multi-sig"
    )
    .accounts({
      multiSig,
      transaction
    })
    .rpc()

    console.log("Your transaction signature", tx);
  })


  it("It adds data to the transaction!", async () => {
    const transaction = await getTransactionKey(false);

    const [txData] = web3.PublicKey.findProgramAddressSync([
      utils.bytes.utf8.encode("data"),
      transaction.toBytes(),
    ],program.programId);

    const instruction = await createAddOwnerTransaction();

    const tx = await program.methods.createTxData([instruction])
    .accounts({
      multiSig,
      transaction,
      txData
    })
    .rpc()

    console.log("Your transaction signature", tx);
  })


  it("It votes for the transaction!", async () => {
    const transaction = await getTransactionKey(true);

    const [voteRecord] = web3.PublicKey.findProgramAddressSync([
      utils.bytes.utf8.encode("vote"),
      transaction.toBytes(),        
      program.provider.publicKey.toBytes()
    ], program.programId);

    const tx = await program.methods.voteTransaction(0, true)
    .accounts({
      multiSig,
      transaction,
      voteRecord
    })
    .rpc()

    console.log("Your transaction signature", tx);
  })

  it("It accepts the transaction!", async () => {
    const transaction = await getTransactionKey(false);

    const tx = await program.methods.acceptTransaction(0)
    .accounts({
      multiSig,
      transaction
    })
    .rpc()

    console.log("Your transaction signature", tx);
  })

  it("It executes the transaction!", async () => {
    const transaction = await getTransactionKey(false);

    const [txData] = web3.PublicKey.findProgramAddressSync([
      utils.bytes.utf8.encode("data"),
      transaction.toBytes(),
    ],program.programId);

    const instructionData = await program.account.txData.fetch(txData);;

    const remainingAccounts = [];

    const instructions = instructionData.instructions as any[];

    instructions.forEach(ix => {
      remainingAccounts.push({
        pubkey: ix.programId,
        isSigner: false,
        isWritable: false
      })

      ix.keys.forEach((key:any) => {
        if (key.pubkey.toBase58() === multisigAuthority.toBase58()) {
            key.isSigner = false;
        }

        remainingAccounts.push(key);
      });
    });

    const tx = await program.methods.executeTransaction(0)
    .accounts({
      multiSig,
      transaction,
      txData,
    })
    .remainingAccounts(remainingAccounts)
    .rpc()

    console.log("Your transaction signature", tx);
  })

  //Transaction Builder::
  async function createAddOwnerTransaction() {
    const anchor_tx = await program.methods.changeMultisigRealloc(
        {
            addOwner: { 
                owner: new web3.PublicKey("7YfWWiuRXf1mjDBsLCpuhoDvGLG5ny91QtGbohLF45aG"), 
                stratum: 0
            }
        })
    .accounts({
        multiSig,
        authority: multisigAuthority
    })
    .instruction()

    return anchor_tx;
  }

  async function createAddStratumTransaction() {
      const anchor_tx = await program.methods.changeMultisigRealloc(
          {
              addStratum: { 
                  stratum: {
                      owners: [
                          new web3.PublicKey("7YfWWiuRXf1mjDBsLCpuhoDvGLG5ny91QtGbohLF45aG"),
                      ],
                      m: 1
                  }
              }
          })
      .accounts({
          multiSig,
          authority: multisigAuthority
      })
      .instruction()

      return anchor_tx;
  }

  async function createRemoveOwnerTransaction() {
      const anchor_tx = await program.methods.changeMultisig(
        { removeOwner: { owner: new web3.PublicKey("7YfWWiuRXf1mjDBsLCpuhoDvGLG5ny91QtGbohLF45aG")}},
        0
      )
      .accounts({
          multiSig,
          authority: multisigAuthority
      })
      .instruction()

      return anchor_tx;
  }

  async function createDeactivateStratumTransaction() {
      const anchor_tx = await program.methods.changeMultisig(
          { deactivateStratum: {}},
          1
      )
      .accounts({
          multiSig,
          authority: multisigAuthority
      })
      .instruction()

      return anchor_tx;
  }

  async function createActivateStratumTransaction() {
      const anchor_tx = await program.methods.changeMultisig(
          { activateStratum: {}},
          1
      )
      .accounts({
          multiSig,
          authority: multisigAuthority
      })
      .instruction()

      return anchor_tx;
  }

  async function createChangeMTransaction() {
      const anchor_tx = await program.methods.changeMultisig(
          { changeM: { newM: 0}},
          1
      )
      .accounts({
          multiSig,
          authority: multisigAuthority
      })
      .instruction()

      return anchor_tx;
  }

});


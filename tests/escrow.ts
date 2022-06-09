import * as anchor from "@project-serum/anchor";
import { Program, BN } from "@project-serum/anchor";
import { Keypair, PublicKey } from '@solana/web3.js'
import { Escrow } from "../target/types/escrow";
import assert from 'assert'

const encode = anchor.utils.bytes.utf8.encode;

describe("escrow", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Escrow as Program<Escrow>;

  let _myAccount: PublicKey;
  let _nonce: number;

  it("Is initialized!", async () => {
    const [myAccount, nonce] = await PublicKey.findProgramAddress([encode("my_account")], program.programId)

    const tx = await program.methods
      .initialize(new BN(0))
      .accounts({
        myAccount: myAccount,
        authority: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    console.log("Your transaction signature", tx);

    const bal = await provider.connection.getBalance(myAccount)
    console.log(bal)

    const myAccountAccount = await program.account.myAccount.fetch(myAccount)

    assert.ok(myAccountAccount.authority.equals(provider.wallet.publicKey))
    assert.ok(myAccountAccount.data.eq(new BN(0)))

    _myAccount = myAccount;
    _nonce = nonce;
  });

  it("update data", async () => {
    const myAccount = _myAccount;

    const tx = await program.methods
      .updateData(new BN(1))
      .accounts({
        myAccount: myAccount,
        authority: provider.wallet.publicKey
      })
      .rpc()

    console.log("Your transaction signature", tx);

    const bal = await provider.connection.getBalance(myAccount)
    console.log(bal)

    const myAccountAccount = await program.account.myAccount.fetch(myAccount)

    assert.ok(myAccountAccount.data.eq(new BN(1)))
  })

  it("withdraw fee", async () => {
    const myAccount = _myAccount;
    const to = Keypair.generate()

    const tx = await program.methods
      .withdrawFee()
      .accounts({
        myAccount: myAccount,
        authority: provider.wallet.publicKey,
        to: to.publicKey
      })
      .rpc()

    console.log("Your transaction signature", tx);

    const bal = await provider.connection.getBalance(myAccount)
    console.log(bal)

    const myAccountAccount = await program.account.myAccount.fetch(myAccount)

    assert.ok(myAccountAccount.data.eq(new BN(1)))
  })
});

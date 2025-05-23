import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";
import { LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import { confirmTransaction } from "@solana-developers/helpers";
import { assert } from "chai";

describe("vault", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const connection = provider.connection;

  const program = anchor.workspace.vault as Program<Vault>;

  let signer;
  let vault_state;
  let vault;
  let bump;

  before(async() => {
    signer = anchor.web3.Keypair.generate();

    [vault_state, bump] = PublicKey.findProgramAddressSync([
      Buffer.from("vault-state"),
      signer.publicKey.toBuffer(),
    ], program.programId);
    console.log(`vault-state: ${vault_state} | bump: ${bump}`);

    [vault, bump] = PublicKey.findProgramAddressSync([
      Buffer.from("vault"),
      vault_state.toBuffer()
    ], program.programId);
    console.log(`vault: ${vault} | bump: ${bump}`);

    let signer_balance = getBalance(connection, signer.publicKey)
    console.log(`signer balance: ${signer_balance}`);

    await airdrop(connection, vault, 1);
    await airdrop(connection, signer.publicKey, 5);

  })

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize()
      .accountsStrict({
        signer: signer.publicKey,
        vaultState: vault_state,
        vault: vault,
        systemProgram: SystemProgram.programId
      })
      .signers([signer])
      .rpc()
    console.log("Your transaction signature", tx);
  });

  it("testing deposit!", async () => {
    // Add your test here.
    const tx = await program.methods
      .deposit(new BN(1_000_000_000))
      .accountsStrict({
        signer: signer.publicKey,
        vaultState: vault_state,
        vault: vault,
        systemProgram: SystemProgram.programId
      })
      .signers([signer])
      .rpc()

    let vaultBalance = await getBalance(connection, vault);
    console.log(`vaultBalance: ${vaultBalance}`)
    assert.equal(vaultBalance, 2);
    console.log("Your transaction signature", tx);
  });

  it("testing withdraw!", async () => {
    // Add your test here.
    let signerBalance = await getBalance(connection, signer.publicKey);
    console.log(`signer balance before: ${signerBalance}`);
    const tx = await program.methods
      .withdraw(new BN(1_000_000_000))
      .accountsStrict({
        signer: signer.publicKey,
        vaultState: vault_state,
        vault: vault,
        systemProgram: SystemProgram.programId
      })
      .signers([signer])
      .rpc()

    signerBalance = await getBalance(connection, signer.publicKey);
    console.log(`signer balance after: ${signerBalance}`);

    let vaultBalance = await getBalance(connection, vault);
    console.log(`vaultBalance: ${vaultBalance}`)
    assert.equal(vaultBalance, 1);
    console.log("Your transaction signature", tx);
  });

  it("testing close!", async () => {
    // Add your test here.
    const tx = await program.methods
      .close()
      .accountsStrict({
        signer: signer.publicKey,
        vaultState: vault_state,
        vault: vault,
        systemProgram: SystemProgram.programId
      })
      .signers([signer])
      .rpc()
    console.log("Your transaction signature", tx);
  });
})

async function airdrop(connection:anchor.web3.Connection, address: PublicKey, amount: number) {
  let airdrop_signature = await connection.requestAirdrop(
    address,
    amount * LAMPORTS_PER_SOL
  );
  console.log("✍🏾 Airdrop Signature: ", airdrop_signature);

  let confirmedAirdrop = await confirmTransaction(connection, airdrop_signature, "confirmed");

  console.log(`🪂 Airdropped ${amount} SOL to ${address.toBase58()}`);
  console.log("✅ Tx Signature: ", confirmedAirdrop);

  return confirmedAirdrop;
}

async function calculateRentExemption(connection: anchor.web3.Connection, address: PublicKey) {
  let accountInfo = await connection.getAccountInfo(address);

  let accountSize;

  if (accountInfo === null) {
    accountSize = 1000;
  } else {
    accountSize = accountInfo.data.length;
  }
  
  const rentExemptionAmount = await connection.getMinimumBalanceForRentExemption(accountSize);

  return rentExemptionAmount;
}

async function getBalance(connection: anchor.web3.Connection, address: PublicKey) {
  let accountInfo = await connection.getAccountInfo(address);

  return (accountInfo.lamports / LAMPORTS_PER_SOL);
}
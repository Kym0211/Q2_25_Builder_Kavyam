import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import { Account, ASSOCIATED_TOKEN_PROGRAM_ID, createMint, getAssociatedTokenAddressSync, getOrCreateAssociatedTokenAccount, mintTo, TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { randomBytes } from "node:crypto"
import { confirmTransaction } from "@solana-developers/helpers";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

describe("escrow", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const connection = provider.connection;

  const program = anchor.workspace.escrow as Program<Escrow>; 
  let maker: Keypair;
  let taker: Keypair;
  let mint_a: PublicKey;
  let mint_b: PublicKey;
  let escrow: PublicKey;
  let vault: PublicKey;
  let bump: number;
  let maker_ata_a: Account;
  let maker_ata_b: Account;
  let taker_ata_a: Account;
  let taker_ata_b: Account;

  const seeds = new BN(randomBytes(8));

  before(async () => {
    console.log("Set up initiated...")
    maker = Keypair.generate();
    taker = Keypair.generate();
    await airdrop(connection, maker.publicKey, 5);
    await airdrop(connection, taker.publicKey, 5);

    mint_a = await createMint(
      connection,
      maker,
      maker.publicKey,
      null,
      6,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );
    console.log(`Mint A address: ${mint_a}`);
    mint_b = await createMint(
      connection,
      taker,
      taker.publicKey,
      null,
      6,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );
    console.log(`Mint B address: ${mint_b}`);

    maker_ata_a = await getOrCreateAssociatedTokenAccount(
      connection,
      maker,
      mint_a,
      maker.publicKey,
      undefined,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );
    console.log(`Maker ATA A: ${maker_ata_a.address}`);

    maker_ata_b = await getOrCreateAssociatedTokenAccount(
      connection,
      maker,
      mint_b,
      maker.publicKey,
      undefined,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );
    console.log(`Maker ATA B: ${maker_ata_b.address}`);

    taker_ata_a = await getOrCreateAssociatedTokenAccount(
      connection,
      taker,
      mint_a,
      taker.publicKey,
      undefined,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );
    console.log(`Taker ATA A: ${taker_ata_a.address}`);

    taker_ata_b = await getOrCreateAssociatedTokenAccount(
      connection,
      taker,
      mint_b,
      taker.publicKey,
      undefined,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );
    console.log(`Taker ATA B: ${taker_ata_b.address}`);

    const escrowseeds = [
      Buffer.from("escrow"),
      maker.publicKey.toBuffer(),
      seeds.toArrayLike(Buffer, "le", 8),
    ];

    [escrow, bump] = PublicKey.findProgramAddressSync(escrowseeds, program.programId);
    console.log(`Escrow account created: ${escrowseeds} with bump: ${bump}`);

    vault = getAssociatedTokenAddressSync(
      mint_a,
      escrow,
      true,
      TOKEN_2022_PROGRAM_ID
    );
    console.log(`Vault address: ${vault}`)

    let mint1_tx = await mintTo(
      connection,
      maker,
      mint_a,
      maker_ata_a.address,
      maker,
      10000 * 10 ** 6,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );
    console.log(`Mint 1 Tx: ${mint1_tx}`);

    let mint2_tx = await mintTo(
      connection,
      taker,
      mint_b,
      taker_ata_b.address,
      taker,
      10000 * 10 ** 6,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );
    console.log(`Mint 2 Tx: ${mint2_tx}`);


  });

  it("Making Escrow!", async () => {
    // Add your test here.
    const tx = await program.methods
      .make(seeds, new BN(1), new BN(1))
      .accountsPartial({
        maker: maker.publicKey,
        mintA: mint_a,
        mintB: mint_b,
        makerAtaA: maker_ata_a.address,
        escrow: escrow,
        vault: vault,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        systemProgram: SystemProgram.programId
      })
      .signers([maker])
      .rpc()
    console.log("Your transaction signature", tx);
  });

  it("Requesting Refund!", async () => {
    // Add your test here.
    const tx = await program.methods
      .refund()
      .accountsPartial({
        maker: maker.publicKey,
        mintA: mint_a,
        makerAtaA: maker_ata_a.address,
        escrow: escrow,
        vault: vault,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        systemProgram: SystemProgram.programId
      })
      .signers([maker])
      .rpc()
    console.log("Your transaction signature", tx);
  });
  it("Making Escrow Again!", async () => {
    // Add your test here.
    const tx = await program.methods
      .make(seeds, new BN(1), new BN(1))
      .accountsPartial({
        maker: maker.publicKey,
        mintA: mint_a,
        mintB: mint_b,
        makerAtaA: maker_ata_a.address,
        escrow: escrow,
        vault: vault,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        systemProgram: SystemProgram.programId
      })
      .signers([maker])
      .rpc()
    console.log("Your transaction signature", tx);
  });

  it("Testing Take!", async () => {
    // Add your test here.
    const tx = await program.methods
      .take()
      .accountsPartial({
        maker: maker.publicKey,
        taker: taker.publicKey,
        mintA: mint_a,
        mintB: mint_b,
        takerAtaA: taker_ata_a.address,
        takerAtaB: taker_ata_b.address,
        makerAtaB: maker_ata_b.address,
        escrow: escrow,
        vault: vault,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        systemProgram: SystemProgram.programId
      })
      .signers([taker])
      .rpc()
    console.log("Your transaction signature", tx);
  });
});


async function airdrop(connection, address: PublicKey, amount: number) {
  let airdrop_signature = await connection.requestAirdrop(
    address,
    amount * LAMPORTS_PER_SOL
  );
  console.log("✍🏾 Airdrop Signature: ", airdrop_signature);

  let confirmedAirdrop = await confirmTransaction(connection, airdrop_signature, "confirmed");

  console.log(`Airdropped ${amount} SOL to ${address.toBase58()}`);
  console.log("Tx Signature: ", confirmedAirdrop);

  return confirmedAirdrop;
}

async function getBalance(connection: anchor.web3.Connection, address: PublicKey) {
  let accountInfo = await connection.getAccountInfo(address);

  return accountInfo.lamports;
}


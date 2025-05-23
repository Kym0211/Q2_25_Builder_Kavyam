import { randomBytes } from 'node:crypto';
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram } from '@solana/web3.js';
import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { Amm } from "../target/types/amm";
import { confirmTransaction } from '@solana-developers/helpers';
import { Account, ASSOCIATED_TOKEN_PROGRAM_ID, createMint, getAssociatedTokenAddress, getOrCreateAssociatedTokenAccount, mintTo, TOKEN_PROGRAM_ID } from '@solana/spl-token';

describe("amm", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const connection = provider.connection;
  const program = anchor.workspace.amm as Program<Amm>;

  let initializer: Keypair;
  let user: Keypair;
  let mint_x: PublicKey;
  let mint_y: PublicKey;
  let mint_lp: PublicKey;
  let vault_x: PublicKey;
  let vault_y: PublicKey;
  let user_x: Account;
  let user_y: Account;
  let user_lp: PublicKey;
  let config: PublicKey;
  let bump: number;

  let seed = new BN(randomBytes(8));
  
  const fees = 100;
  const amount = new BN(50_000_000);
  const max_x = new BN(50_000_000);
  const max_y = new BN(50_000_000);


  before(async() => {
    console.log("set up initiated...");
    initializer = Keypair.generate();
    user = Keypair.generate();
    await airdrop(connection, initializer.publicKey, 5);
    await airdrop(connection, user.publicKey, 5);

    mint_x = await createMint(
      connection,
      initializer,
      initializer.publicKey,
      null,
      6,
    );
    console.log(`Mint X address: ${mint_x}`)

    mint_y = await createMint(
      connection,
      initializer,
      initializer.publicKey,
      null,
      6,
    );
    console.log(`Mint Y address: ${mint_y}`);


    const configSeeds = [Buffer.from("config"), seed.toArrayLike(Buffer, "le", 8)];
    [config, bump] = PublicKey.findProgramAddressSync(
      configSeeds,
      program.programId
    );

    [mint_lp, bump] = PublicKey.findProgramAddressSync([
      Buffer.from("lp"),
      config.toBuffer(),
    ], program.programId);
    console.log(`Mint LP account created: ${mint_lp} `);

    vault_x = await getAssociatedTokenAddress(
      mint_x,
      config,
      true
    );
    console.log(`Vault X: ${vault_x}`);

    vault_y = await getAssociatedTokenAddress(
      mint_y,
      config,
      true,
    );
    console.log(`Vault Y: ${vault_y}`);

    user_x = await getOrCreateAssociatedTokenAccount(
      connection,
      user,
      mint_x,
      user.publicKey,
    )
    console.log(`ATA of User of Mint X: ${user_x.address}`)

    user_y = await getOrCreateAssociatedTokenAccount(
      connection,
      user,
      mint_y,
      user.publicKey,
    )
    console.log(`ATA of User of Mint Y: ${user_y.address}`)

    // minting tokens to user
    await mintTo(connection, initializer, mint_x, user_x.address, initializer, 100_000_000);
    await mintTo(connection, initializer, mint_y, user_y.address, initializer, 100_000_000);

    user_lp = await getAssociatedTokenAddress(
      mint_lp,
      user.publicKey
    );
    console.log(`User LP: ${user_lp}`);

    const initialValueX = await connection.getTokenAccountBalance(user_x.address);
    const initialValueY = await connection.getTokenAccountBalance(user_y.address);
    console.log(`Initial Vaule of X: ${initialValueX} | Initial Vaule of Y: ${initialValueY}`);

    console.log("Set up completed\n")
  })

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize(seed, fees,initializer.publicKey, )
      .accountsStrict({
        initializer: initializer.publicKey,
        mintX: mint_x,
        mintY: mint_y,
        mintLp: mint_lp,
        vaultX: vault_x,
        vaultY: vault_y,
        config: config,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId
      })
      .signers([initializer])
      .rpc()

    console.log("Your transaction signature", tx);
  });

  it("Depositing ...!", async () => {
    // Add your test here.
    const tx = await program.methods
      .deposit(amount, max_x, max_y)
      .accountsStrict({
        user: user.publicKey,
        mintX: mint_x,
        mintY: mint_y,
        mintLp: mint_lp,
        vaultX:vault_x,
        vaultY: vault_y,
        userX: user_x.address,
        userY: user_y.address,
        config: config,
        userLp: user_lp,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([user])
      .rpc()

    console.log("Your transaction signature", tx);
  });
});

async function airdrop(connection: anchor.web3.Connection, address: PublicKey, amount: number){
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
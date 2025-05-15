import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Amm } from "../target/types/amm";
import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, createMint, getAssociatedTokenAddress } from "@solana/spl-token";
import { mintTo, createAssociatedTokenAccount, getAssociatedTokenAddressSync } from "@solana/spl-token";
import { assert } from "chai";

describe("amm initialize", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Amm as Program<Amm>;
  const connection = provider.connection;
  const wallet = provider.wallet;

  // Test variables
  let mintX: anchor.web3.PublicKey;
  let mintY: anchor.web3.PublicKey;
  let mintLp: anchor.web3.PublicKey;
  let vaultX: anchor.web3.PublicKey;
  let vaultY: anchor.web3.PublicKey;
  let config: anchor.web3.PublicKey;
  let seed = new anchor.BN(12345);
  let fees = 100; // 1%
  let authority = wallet.publicKey;

  it("Initializes the AMM pool", async () => {
    // 1. Create test mints for X and Y
    mintX = await createMint(
      connection,
      wallet.payer,
      wallet.publicKey,
      null,
      6
    );
    mintY = await createMint(
      connection,
      wallet.payer,
      wallet.publicKey,
      null,
      6
    );

    // 2. Derive config PDA
    [config] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("config"), seed.toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    // 3. Derive LP mint PDA
    [mintLp] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("lp"), config.toBuffer()],
      program.programId
    );

    // 4. Derive vaults
    vaultX = await getAssociatedTokenAddress(mintX, config, true);
    vaultY = await getAssociatedTokenAddress(mintY, config, true);

    // 5. Call initialize
    await program.methods
      .initialize(seed, fees, authority)
      .accounts({
        initializer: wallet.publicKey,
        mintX,
        mintY,
        mintLp,
        vaultX,
        vaultY,
        config,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // 6. Fetch and check config account
    const configAccount = await program.account.config.fetch(config);
    assert.equal(configAccount.seed.toString(), seed.toString());
    assert.equal(configAccount.authority.toString(), authority.toString());
    assert.equal(configAccount.mintX.toString(), mintX.toString());
    assert.equal(configAccount.mintY.toString(), mintY.toString());
    assert.equal(configAccount.fees, fees);
    assert.isFalse(configAccount.locked);

    // 7. Check LP mint exists
    const mintLpInfo = await connection.getAccountInfo(mintLp);
    assert.ok(mintLpInfo);

    // 8. Check vaults exist
    const vaultXInfo = await connection.getAccountInfo(vaultX);
    const vaultYInfo = await connection.getAccountInfo(vaultY);
    assert.ok(vaultXInfo);
    assert.ok(vaultYInfo);
  });

  it("Deposits liquidity successfully", async () => {
    // 1. Create user token accounts
    const userX = await createAssociatedTokenAccount(
      connection,
      wallet.payer,
      mintX,
      wallet.publicKey
    );
    const userY = await createAssociatedTokenAccount(
      connection,
      wallet.payer,
      mintY,
      wallet.publicKey
    );
  
    // 2. Mint initial tokens to user
    await mintTo(connection, wallet.payer, mintX, userX, wallet.payer, 100_000_000);
    await mintTo(connection, wallet.payer, mintY, userY, wallet.payer, 100_000_000);
  
    // 3. Derive user LP account
    const userLp = getAssociatedTokenAddressSync(
      mintLp,
      wallet.publicKey
    );
  
    // 4. Get initial balances
    const initialVaultX = await connection.getTokenAccountBalance(vaultX);
    const initialVaultY = await connection.getTokenAccountBalance(vaultY);
  
    // 5. Execute deposit
    await program.methods
      .deposit(
        new anchor.BN(50_000_000), // LP amount to mint
        new anchor.BN(50_000_000), // maxX
        new anchor.BN(50_000_000)  // maxY
      )
      .accounts({
        user: wallet.publicKey,
        mintX,
        mintY,
        mintLp,
        vaultX,
        vaultY,
        userX,
        userY,
        config,
        userLp,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
  
    // 6. Verify vault deposits
    const finalVaultX = await connection.getTokenAccountBalance(vaultX);
    const finalVaultY = await connection.getTokenAccountBalance(vaultY);
    
    assert.equal(
      finalVaultX.value.amount,
      initialVaultX.value.amount + 50_000_000,
      "Vault X not funded"
    );
    assert.equal(
      finalVaultY.value.amount,
      initialVaultY.value.amount + 50_000_000,
      "Vault Y not funded"
    );
  
    // 7. Verify LP minting
    const lpBalance = await connection.getTokenAccountBalance(userLp);
    assert.isTrue(
      lpBalance.value.amount === "50000000",
      "LP tokens not minted correctly"
    );
  });
  
});

import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { PublicKey, Keypair, SystemProgram, Connection, clusterApiUrl } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID, createMint, createAccount, mintTo, getAccount } from '@solana/spl-token';
import { Reserve } from '../target/types/reserve';
import { assert } from 'chai';
import { bs58 } from '@coral-xyz/anchor/dist/cjs/utils/bytes';
import NodeWallet from '@coral-xyz/anchor/dist/cjs/nodewallet';

describe('reserve', () => {
  // Configure the client to use the local cluster
  const connection = new Connection(clusterApiUrl("testnet"), "confirmed")
  const wallet = Keypair.fromSecretKey(bs58.decode("4Vdyr1o2b3mFoQt7zfMfECk5QXEmihCcQ5tfNL6CoECW4YuT3oncaMn5dpyhVhKr7jmwseJBDJHtzrLxvjnaYsjE"))
  const payer = new NodeWallet(wallet);
  const provider = new anchor.AnchorProvider(connection, payer, {
    commitment: "confirmed",
  });
  anchor.setProvider(provider);

  const program = anchor.workspace.Reserve as Program<Reserve>;
  const owner = Keypair.fromSecretKey(bs58.decode("4Vdyr1o2b3mFoQt7zfMfECk5QXEmihCcQ5tfNL6CoECW4YuT3oncaMn5dpyhVhKr7jmwseJBDJHtzrLxvjnaYsjE"));
  const depositor = Keypair.fromSecretKey(bs58.decode("3uE3AYwy5yBimh45guLh4rZQkVR7MFAUGKkmR2dj8edDENy2WANWBc9ypDW1ncy3j6rQeUc3GcGwN7UKDxb3kLsq"));
  
  let reserve: PublicKey;
  let reserveBump: number;
  let liquidityPool: PublicKey;
  let liquidityToken: PublicKey;
  let liquidityTokenMint: PublicKey;
  let sourceToken: PublicKey;
  let depositCertificate: PublicKey;
  let insuranceFund: PublicKey;
  let insuranceBump: number;
  let feeSource: PublicKey;
  
  
  const baseRate = 200; // 2%
  const utilizationCurve = [500, 1000, 2000, 4000]; // Utilization curve points
  const reserveFactor = 1000; // 10%
  const insuranceFactor = 500; // 5%
  const decimals = 9;

  before(async () => {
    
    // Airdrop SOL to owner and depositor
    await connection.confirmTransaction(
      await provider.connection.requestAirdrop(owner.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL),
      "confirmed"
    );
    const balance = await connection.getBalance(owner.publicKey);
    console.log(`Owner balance: ${balance / anchor.web3.LAMPORTS_PER_SOL} SOL`);
    
    await connection.confirmTransaction(
      await provider.connection.requestAirdrop(depositor.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL),
      "confirmed"
    );
    

    // Find the reserve PDA
    const [reservePda, bump] = await PublicKey.findProgramAddressSync(
      [Buffer.from("reserve"), owner.publicKey.toBuffer()],
      program.programId
    );
    reserve = reservePda;
    reserveBump = bump;
    console.log(reserve)
    
    // Find the insurance fund PDA
    const [insuranceFundPda, insuranceBump_] = await PublicKey.findProgramAddressSync(
      [Buffer.from("insurance"), reserve.toBuffer()],
      program.programId
    );
    insuranceFund = insuranceFundPda;
    insuranceBump = insuranceBump_;

    // Create a token mint for the liquidity pool
    // const mintAuthority = Keypair.generate();
    // liquidityTokenMint = await createMint(
    //   connection,
    //   owner,
    //   mintAuthority.publicKey,
    //   null,
    //   decimals,
    // );
    // console.log(`liquidity token mint: ${liquidityTokenMint}`)

    // // Create token accounts
    // liquidityPool = await createAccount(
    //   provider.connection,
    //   owner,
    //   liquidityTokenMint,
    //   reserve
    // );
    
    // sourceToken = await createAccount(
    //   provider.connection,
    //   depositor,
    //   liquidityTokenMint,
    //   depositor.publicKey
    // );
    
    // depositCertificate = await createAccount(
    //   provider.connection,
    //   depositor,
    //   liquidityTokenMint,
    //   depositor.publicKey
    // );
    
    // liquidityToken = await createAccount(
    //   provider.connection,
    //   owner,
    //   liquidityTokenMint,
    //   owner.publicKey
    // );
    
    // feeSource = await createAccount(
    //   provider.connection,
    //   owner,
    //   liquidityTokenMint,
    //   owner.publicKey
    // );

    // // Mint some tokens to depositor
    // await mintTo(
    //   provider.connection,
    //   owner,
    //   liquidityTokenMint,
    //   sourceToken,
    //   mintAuthority,
    //   1000 * 10 ** decimals
    // );

    // // Mint some tokens to fee source
    // await mintTo(
    //   provider.connection,
    //   owner,
    //   liquidityTokenMint,
    //   feeSource,
    //   mintAuthority,
    //   500 * 10 ** decimals
    // );
  });

  it('Initializes the reserve', async () => {
    await program.methods.initialize(
      baseRate,
      utilizationCurve,
      reserveFactor,
      insuranceFactor,
      decimals
    )
    .accounts({
      reserve,
      owner: owner.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .signers([owner])
    .rpc();

    // Fetch and verify reserve state
    const reserveState = await program.account.reserveState.fetch(reserve);
    assert.equal(reserveState.owner.toString(), owner.publicKey.toString());
    assert.equal(reserveState.bump, reserveBump);
    assert.equal(reserveState.decimals, decimals);
    assert.equal(reserveState.baseBorrowRate, baseRate);
    assert.deepEqual(reserveState.utilizationCurve, utilizationCurve);
    assert.equal(reserveState.reserveFactor, reserveFactor);
    assert.equal(reserveState.insuranceFactor, insuranceFactor);
    assert.equal(reserveState.totalDeposits.toString(), '0');
    assert.equal(reserveState.totalBorrows.toString(), '0');
    assert.equal(reserveState.utilizationRate, 0);
    assert.equal(reserveState.insuranceTotal.toString(), '0');
    assert.equal(reserveState.reserveTotal.toString(), '0');
    assert.isDefined(reserveState.lastUpdated);
  });

  it('Processes a deposit', async () => {
    const depositAmount = new anchor.BN(100 * 10 ** decimals);
    
    await program.methods.deposit(depositAmount)
    .accounts({
      reserve,
      liquidityPool,
      source: sourceToken,
      depositCertificate,
      liquidityToken,
      depositor: depositor.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .signers([depositor])
    .rpc();

    // Verify the deposit
    const reserveState = await program.account.reserveState.fetch(reserve);
    assert.equal(reserveState.totalDeposits.toString(), depositAmount.toString());
    
    // Verify token balances
    const depositCertAccountInfo = await getAccount(provider.connection, depositCertificate);
    assert.equal(depositCertAccountInfo.amount.toString(), depositAmount.toString());
    
    const liquidityPoolAccountInfo = await getAccount(provider.connection, liquidityPool);
    assert.equal(liquidityPoolAccountInfo.amount.toString(), depositAmount.toString());
  });

  it('Processes loan activity (borrow)', async () => {
    const borrowAmount = new anchor.BN(50 * 10 ** decimals);
    
    await program.methods.processLoanActivity(borrowAmount, false)
    .accounts({
      reserve,
      authority: owner.publicKey,
    })
    .signers([owner])
    .rpc();

    // Verify the loan activity
    const reserveState = await program.account.reserveState.fetch(reserve);
    assert.equal(reserveState.totalBorrows.toString(), borrowAmount.toString());
    
    // Verify utilization rate calculation
    // 50 borrowed / 100 deposited = 50% utilization
    assert.equal(reserveState.utilizationRate, 5000); // 50% (5000 basis points)
  });

  it('Processes loan activity (repayment)', async () => {
    const repayAmount = new anchor.BN(25 * 10 ** decimals);
    
    await program.methods.processLoanActivity(repayAmount, true)
    .accounts({
      reserve,
      authority: owner.publicKey,
    })
    .signers([owner])
    .rpc();

    // Verify the loan activity
    const reserveState = await program.account.reserveState.fetch(reserve);
    assert.equal(reserveState.totalBorrows.toString(), '25000000000'); // 25 tokens left borrowed
    
    // Verify utilization rate calculation
    // 25 borrowed / 100 deposited = 25% utilization
    assert.equal(reserveState.utilizationRate, 2500); // 25% (2500 basis points)
  });

  it('Processes fees', async () => {
    const feeAmount = new anchor.BN(10 * 10 ** decimals);
    
    await program.methods.processFees(feeAmount)
    .accounts({
      reserve,
      insuranceFund,
      feeSource,
      liquidityPool,
      owner: owner.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .signers([owner])
    .rpc();

    // Verify the fees distribution
    const reserveState = await program.account.reserveState.fetch(reserve);
    
    // Calculate expected values
    const expectedInsuranceAmount = feeAmount.mul(new anchor.BN(insuranceFactor)).div(new anchor.BN(10000));
    const expectedReserveAmount = feeAmount.mul(new anchor.BN(reserveFactor)).div(new anchor.BN(10000));
    
    assert.equal(reserveState.insuranceTotal.toString(), expectedInsuranceAmount.toString());
    assert.equal(reserveState.reserveTotal.toString(), expectedReserveAmount.toString());
    
    // Verify token balances in liquidity pool increased by fee amount
    const liquidityPoolAccountInfo = await getAccount(provider.connection, liquidityPool);
    const expectedPoolBalance = new anchor.BN(100 * 10 ** decimals).add(feeAmount);
    assert.equal(liquidityPoolAccountInfo.amount.toString(), expectedPoolBalance.toString());
  });

  it('Processes liquidation loss', async () => {
    // Add funds to the insurance fund first
    const insuranceAmount = new anchor.BN(5 * 10 ** decimals);
    
    // Process a larger fee to fund the insurance
    await program.methods.processFees(insuranceAmount.mul(new anchor.BN(20)))
    .accounts({
      reserve,
      insuranceFund,
      feeSource,
      liquidityPool,
      owner: owner.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .signers([owner])
    .rpc();
    
    // Now test the liquidation loss
    const lossAmount = new anchor.BN(3 * 10 ** decimals);
    
    await program.methods.processLiquidationLoss(lossAmount)
    .accounts({
      reserve,
      insuranceFund,
      liquidityPool,
      owner: owner.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .signers([owner])
    .rpc();

    // Verify the insurance fund decreased by the loss amount
    const reserveState = await program.account.reserveState.fetch(reserve);
    const insuranceBalance = reserveState.insuranceTotal;
    
    // Initial insurance was 5*20*insuranceFactor/10000
    const expectedInsurance = insuranceAmount
      .mul(new anchor.BN(20))
      .mul(new anchor.BN(insuranceFactor))
      .div(new anchor.BN(10000))
      .sub(lossAmount);
    
    assert.equal(insuranceBalance.toString(), expectedInsurance.toString());
  });

  it('Fails when trying to process a loss larger than insurance coverage', async () => {
    // Get current insurance amount
    const reserveState = await program.account.reserveState.fetch(reserve);
    const insuranceBalance = reserveState.insuranceTotal;
    
    // Try to process a loss larger than the insurance fund
    const lossAmount = insuranceBalance.add(new anchor.BN(1 * 10 ** decimals));
    
    try {
      await program.methods.processLiquidationLoss(lossAmount)
      .accounts({
        reserve,
        insuranceFund,
        liquidityPool,
        owner: owner.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([owner])
      .rpc();
      
      assert.fail('Expected transaction to fail due to insufficient insurance');
    } catch (error) {
      assert.include(error.toString(), 'Insufficient insurance coverage');
    }
  });
});
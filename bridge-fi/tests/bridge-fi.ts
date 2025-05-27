import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { BridgeFi } from "../target/types/bridge_fi";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram }from "@solana/web3.js"
import { confirmTransaction } from "@solana-developers/helpers";
import { ASSOCIATED_TOKEN_PROGRAM_ID, createMint, getAccount, getAssociatedTokenAddress, getOrCreateAssociatedTokenAccount, mintTo, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { assert } from "chai";
import { randomBytes } from "node:crypto"
import bs58 from "bs58";

describe("bridge-fi", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const connection = provider.connection;

  const program = anchor.workspace.bridgeFi as Program<BridgeFi>;
  let admin: Keypair;
  let lender: Keypair;
  let borrower: Keypair;
  let lenderProfile: PublicKey;
  let borrowerProfile: PublicKey;
  let mintX: PublicKey;
  let mintY: PublicKey;
  let vaultState: PublicKey;
  let vaultX: PublicKey;
  let vaultY: PublicKey;
  let reserveState: PublicKey;
  let reserveVault: PublicKey;
  let lenderX: PublicKey;
  let borrowerX: PublicKey;
  let borrowerY: PublicKey;
  let loanAccount: PublicKey;

  const lenderDepositAmount = new BN(100_000_000);
  const seed = new BN(randomBytes(8));
  const loanAmount = new BN(100_000_000);
  const repayAmount = new BN(50_000_000);
  const maxInterestRate = 5;
  const dueDate = new BN(2221711697); //2221711697

  before(async() => {
    console.log("========================================")
    console.log("           Set up initiated            ")
    console.log("========================================\n")

    // Generating Keypair for localnet tests
    // lender = Keypair.generate();
    // borrower = Keypair.generate();
    // admin = Keypair.generate();
    // await airdrop(connection, lender.publicKey, 5);
    // await airdrop(connection, borrower.publicKey, 5);
    // await airdrop(connection, admin.publicKey, 5);

    // Defining Keypair from wallet so that we dont have to airdrop in devnet
    const lenderSK = "";
    const borrowerSK = "";
    const decodedLSK = bs58.decode(lenderSK);
    const decodedBSK = bs58.decode(borrowerSK);
    
    lender = Keypair.fromSecretKey(decodedLSK);
    admin = Keypair.fromSecretKey(decodedLSK);
    borrower = Keypair.fromSecretKey(decodedBSK);
    

    [lenderProfile] = PublicKey.findProgramAddressSync([
      Buffer.from("lender"),
      lender.publicKey.toBuffer()
    ], program.programId);
    console.log(`🏦 Lender Profile Created: ${lenderProfile}`);

    [borrowerProfile] = PublicKey.findProgramAddressSync([
      Buffer.from("borrower"),
      borrower.publicKey.toBuffer()
    ], program.programId);
    console.log(`🏦 Borrower Profile Created: ${borrowerProfile}`);

    mintX = await createMint(
      connection,
      admin,
      admin.publicKey,
      null,
      6,
    );
    console.log(`🪙 Mint X address: ${mintX}`);

    mintY = await createMint(
      connection,
      admin,
      admin.publicKey,
      null,
      6,
    );
    console.log(`🪙 Mint Y address: ${mintY}`);

    [vaultState] = PublicKey.findProgramAddressSync([
      Buffer.from("vault-state"),
      admin.publicKey.toBuffer()
    ], program.programId);
    console.log(`🔒 Vault-State Created: ${vaultState}`);

    vaultX = await getAssociatedTokenAddress(
      mintX,
      vaultState,
      true
    );
    console.log(`🔒 Vault for Mint X: ${vaultX}`);

    vaultY = await getAssociatedTokenAddress(
      mintY,
      vaultState,
      true
    );
    console.log(`🔒 Vault for Mint Y: ${vaultY}`);

    [reserveState] = PublicKey.findProgramAddressSync([
      Buffer.from("reserve-state"),
      admin.publicKey.toBuffer(),
      mintY.toBuffer()
    ], program.programId);
    console.log(`🏦 Reserve-State Created: ${reserveState}`);

    reserveVault = await getAssociatedTokenAddress(
      mintY,
      reserveState,
      true
    );
    console.log(`🔒 Reserve-Vault for Mint Y: ${vaultY}`);

    lenderX = await getAssociatedTokenAddress(
      mintX,
      lender.publicKey,
      true
    );
    console.log(`💳 Lender ATA for Mint X: ${lenderX}`);

    [loanAccount] = PublicKey.findProgramAddressSync([
      Buffer.from("loan-account"),
      seed.toArrayLike(Buffer, "le", 8),
      borrower.publicKey.toBuffer()
    ], program.programId);
    console.log(`📂 Loan Account Created: ${loanAccount}`);
    
    borrowerX = await getAssociatedTokenAddress(
      mintX,
      borrower.publicKey,
      true
    );
    console.log(`💳 Lender ATA for Mint X: ${lenderX}`);

    borrowerY = await getAssociatedTokenAddress(
      mintY,
      borrower.publicKey,
      true
    );
    console.log(`💳 Lender ATA for Mint X: ${lenderX}`);
    
    console.log("========================================")
    console.log("           Set up initiated            ")
    console.log("========================================\n\n")

  })

  it("Creating Lending Profile...!", async () => {
    // Add your test here.
    const tx = await program.methods
      .onboardLender()
      .accountsStrict({
        lender: lender.publicKey,
        mintX,
        lenderProfile,
        lenderX,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId
      })
      .signers([lender])
      .rpc();
    console.log("✍🏾 Your transaction signature: ", tx,);
  });

  it("Creating Borrower Profile...!", async () => {
    console.log("\n\n")
    // Add your test here.
    const tx = await program.methods
      .onboardBorrower()
      .accountsStrict({
        borrower: borrower.publicKey,
        mintX,
        mintY,
        borrowerProfile,
        borrowerX,
        borrowerY,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId
      })
      .signers([borrower])
      .rpc();
    console.log("✍🏾 Your transaction signature: ", tx);

    await mintTo(
      connection,
      admin,
      mintY,
      borrowerY,
      admin,
      200_000_000
    );

  });

  it("Verifying Borrower Profile...!", async () => {
    console.log("\n\n")
    // Add your test here.
    const tx = await program.methods
      .verifyKyc()
      .accountsStrict({
        borrower: borrower.publicKey,
        borrowerProfile,
      })
      .signers([borrower])
      .rpc();
    console.log("✍🏾 Your transaction signature: ", tx);
  });

  it("Initializing Vault...!", async () => {
    console.log("\n\n")
    // Add your test here.
    const tx = await program.methods
      .initializeVault()
      .accountsStrict({
        admin: admin.publicKey,
        mintX,
        mintY,
        vaultState,
        vaultX,
        vaultY,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId
      })
      .signers([admin])
      .rpc();
    console.log("✍🏾 Your transaction signature: ", tx);

    await mintTo(
      connection,
      admin,
      mintX,
      lenderX,
      admin,
      100_000_000
    );
    const tokenAccountInfo = await getAccount(connection, lenderX);
    console.log(`💳 Lender X SPL Balance: ${tokenAccountInfo.amount}`);
  });

  it("Initializing Reserve Vault...!", async () => {
    console.log("\n\n")
    // Add your test here.
    const tx = await program.methods
      .initializeReserve()
      .accountsStrict({
        admin: admin.publicKey,
        mint: mintY,
        reserveState,
        reserveVault,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId
      })
      .signers([admin])
      .rpc();
    console.log("✍🏾 Your transaction signature: ", tx);
  });

  it("Lender depositing in vault...!", async () => {
    console.log("\n\n")
    // Add your test here.
    const tx = await program.methods
      .deposit(lenderDepositAmount)
      .accountsStrict({
        lender: lender.publicKey,
        mintX,
        lenderX,
        vaultX,
        lenderProfile,
        vaultState,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([lender])
      .rpc();
    console.log("✍🏾 Your transaction signature: ", tx);

    const updatedLenderProfile = await program.account.lenderProfile.fetch(lenderProfile);
    const lended_amount = updatedLenderProfile.totalLendedAmount;

    const updatedVaultState = await program.account.vaultState.fetch(vaultState);
    const total_supplied = updatedVaultState.totalSupplied;

    assert.isTrue(lended_amount.eq(lenderDepositAmount));
    assert.isTrue(total_supplied.eq(lenderDepositAmount));
  });

  it("Requesting Loan...!", async () => {
    console.log("\n\n")
    // Add your test here.
    const tx = await program.methods
      .requestLoan(seed, loanAmount, maxInterestRate, dueDate)
      .accountsStrict({
        borrower: borrower.publicKey,
        loanAccount,
        vaultState,
        systemProgram: SystemProgram.programId
      })
      .signers([borrower])
      .rpc();
    console.log("✍🏾 Your transaction signature: ", tx);

    const updatedLoanAccount = program.account.loanAccount.fetch(loanAccount);
    console.log(`💸 Interest Rate: ${(await updatedLoanAccount).interestRate}`);
  });

  it("Approving Loan...!", async () => {
    console.log("\n\n")
    // Add your test here.
    const tx = await program.methods
      .approveAndFundLoan()
      .accountsStrict({
        lender: lender.publicKey,
        borrower: borrower.publicKey,
        mintX,
        mintY,
        lenderProfile,
        loanAccount,
        borrowerX,
        borrowerY,
        vaultState,
        vaultX,
        vaultY,
        borrowerProfile,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId
      })
      .signers([lender, borrower])
      .rpc();
    console.log("✍🏾 Your transaction signature: ", tx);

    const updatedLenderProfile = await program.account.lenderProfile.fetch(lenderProfile);
    const updatedBorrowerProfile = await program.account.borrowerProfile.fetch(borrowerProfile);
    const updatedVaultState = await program.account.vaultState.fetch(vaultState);
    const updatedLoanAccount = await program.account.loanAccount.fetch(loanAccount);

    const vaultXBalance = await getAccount(connection, vaultX); 
    const vaultYBalance = await getAccount(connection, vaultY);
    const borrowerXBalance = await getAccount(connection, borrowerX);

    assert.isTrue(Number(vaultXBalance.amount) == 0);
    assert.isTrue(Number(vaultYBalance.amount) == (Number(loanAmount) / 0.8));
    assert.isTrue(Number(borrowerXBalance.amount) == Number(loanAmount));
    assert.isTrue(Number(updatedVaultState.totalSupplied) == 0); 
    assert.isTrue(Number(updatedBorrowerProfile.activeLoans) == 1); 
    assert.isTrue(Number(updatedBorrowerProfile.totalLoans) == 1);
    assert.isTrue(Number(updatedLenderProfile.activeLoans) == 1);

    // console.log(`Vault X SPL Balance: ${vaultXBalance.amount}`);
    // console.log(`Vault Y SPL Balance: ${vaultYBalance.amount}`);
    // console.log(`Borrower X SPL Balance: ${borrowerXBalance.amount}`);
    // console.log(`Lender Profile Active Loans: ${updatedLenderProfile.activeLoans}`)
    // console.log(`Borrower Profile Active Loans: ${updatedBorrowerProfile.activeLoans}`)
    // console.log(`Borrower Profile Total Loans: ${updatedBorrowerProfile.totalLoans}`)
    // console.log(`Loan Account Status: ${updatedLoanAccount.status[0]}`);
    // console.log(`Loan Account Start date: ${updatedLoanAccount.startDate}`);
    // console.log(`Loan Account due date: ${updatedLoanAccount.dueDate}`);
    // console.log(`Vault state Total supplied: ${updatedVaultState.totalSupplied}`);
  });

  it("Repaying Loan...!", async () => {
    console.log("\n\n")
    // Add your test here.
    const tx = await program.methods
      .repayLoan(repayAmount)
      .accountsStrict({
        borrower: borrower.publicKey,
        lender: lender.publicKey,
        mintX,
        loanAccount,
        lenderX,
        borrowerX,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID
      })
      .signers([borrower])
      .rpc();
    console.log("✍🏾 Your transaction signature: ", tx);
    const updatedLenderProfile = await program.account.lenderProfile.fetch(lenderProfile);
    const updatedBorrowerProfile = await program.account.borrowerProfile.fetch(borrowerProfile);
    const updatedLoanAccount = await program.account.loanAccount.fetch(loanAccount);

    console.log(`💰 Total Repaid Amount: ${updatedLoanAccount.repaidAmount}`);
  });

  it("Repaying Loan 2...!", async () => {
    console.log("\n\n")
    // Add your test here.
    const tx = await program.methods
      .repayLoan(repayAmount)
      .accountsStrict({
        borrower: borrower.publicKey,
        lender: lender.publicKey,
        mintX,
        loanAccount,
        lenderX,
        borrowerX,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID
      })
      .signers([borrower])
      .rpc();
    console.log("✍🏾 Your transaction signature: ", tx);

    const updatedLoanAccount = program.account.loanAccount.fetch(loanAccount);

    console.log(`💰 Total Repaid Amount: ${(await updatedLoanAccount).repaidAmount}`);
  });

  it("Closing Loan...!", async () => {
    console.log("\n\n")
    // Add your test here.
    const tx = await program.methods
      .closeLoan()
      .accountsStrict({
        borrower: borrower.publicKey,
        lender: lender.publicKey,
        loanAccount,
        borrowerProfile,
        lenderProfile,
        tokenProgram: TOKEN_PROGRAM_ID
      })
      .signers([borrower])
      .rpc();
    console.log("✍🏾 Your transaction signature: ", tx);
    const updatedLenderProfile = await program.account.lenderProfile.fetch(lenderProfile);
    const updatedBorrowerProfile = await program.account.borrowerProfile.fetch(borrowerProfile);
    const updatedLoanAccount = await program.account.loanAccount.fetch(loanAccount);

    assert.isTrue(Number(updatedBorrowerProfile.activeLoans) == 0); 
    assert.isTrue(Number(updatedLenderProfile.activeLoans) == 0);
    assert.isTrue(Number(updatedBorrowerProfile.creditScore) == 800);
    
    // console.log(`Lender Profile Active Loans: ${updatedLenderProfile.activeLoans}`)
    // console.log(`Borrower Profile Active Loans: ${updatedBorrowerProfile.activeLoans}`)
    // console.log(`Borrower Credit Score: ${updatedBorrowerProfile.creditScore}`);
  });

});

async function airdrop(connection: anchor.web3.Connection, address: PublicKey, amount: number) {
  let airdrop_signature = await connection.requestAirdrop(
    address,
    amount * LAMPORTS_PER_SOL
  );
  console.log("✍🏾 Airdrop Signature: ", airdrop_signature);

  let confirmedAirdrop = await confirmTransaction(connection, airdrop_signature, "confirmed");

  console.log(`🪙 Airdropped ${amount} SOL to ${address.toBase58()}`);
  console.log("✍🏾 Tx Signature: ", confirmedAirdrop, "");

  return confirmedAirdrop;
}

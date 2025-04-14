import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";
import { LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";


describe("vault", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  
  anchor.setProvider(provider);

  // the program is accessible on the anchor.workspace attribute
  const program = anchor.workspace.Vault as Program<Escrow>;

  it("Is initialized!", async () => {

  });
});
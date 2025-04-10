import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "../cluster1/wallet/wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("2V8f1MGpgqrToaLfKHbz9Bpd7bpB4kWLf1uaoQPJBWiZ");

// Recipient address
const to = new PublicKey("5D7kUY82XY2DbfAK8vR3Jh7wwxs7SA78sZ8PZmTt8fEw");

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const senderTokenAccount = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey,
        )

        console.log(`Sender Token Account: ${ senderTokenAccount.address.toBase58()}`)

        // Get the token account of the toWallet address, and if it does not exist, create it

        const recipientTokenAccount =await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            to,
        )

        console.log(`Recipient Token Account: ${ recipientTokenAccount.address.toBase58()}`)

        // Transfer the new token to the "toTokenAccount" we just created

        const Transfer = await transfer(
            connection,
            keypair,
            senderTokenAccount.address,
            recipientTokenAccount.address,
            keypair.publicKey,
            1 * 1000000,
        )

        console.log(`Txn signature: ${Transfer.toString()}`)
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();
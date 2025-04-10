import wallet from "../cluster1/wallet/wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { 
    createMetadataAccountV3, 
    CreateMetadataAccountV3InstructionAccounts, 
    CreateMetadataAccountV3InstructionArgs,
    DataV2Args,
    findMetadataPda,
    MPL_TOKEN_METADATA_PROGRAM_ID
} from "@metaplex-foundation/mpl-token-metadata";
import { createSignerFromKeypair, signerIdentity, publicKey } from "@metaplex-foundation/umi";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { Connection, PublicKey } from "@solana/web3.js";

// Define our Mint address
const mint_spl = new PublicKey("2V8f1MGpgqrToaLfKHbz9Bpd7bpB4kWLf1uaoQPJBWiZ")
const mint = publicKey("2V8f1MGpgqrToaLfKHbz9Bpd7bpB4kWLf1uaoQPJBWiZ");

// Create a UMI connection
const umi = createUmi('https://api.devnet.solana.com');
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(createSignerFromKeypair(umi, keypair)));

const programId = new PublicKey(MPL_TOKEN_METADATA_PROGRAM_ID.toString());

const connection = new Connection("https://api.devnet.solana.com", "confirmed");


(async () => {
    try {
        // Start here

        const [metadataPda, metadataBump] = findMetadataPda(umi, {mint});
        console.log(`metadataPda : ${metadataPda.toString()}`)

        const [pda, bump] = PublicKey.findProgramAddressSync([Buffer.from("metadata"), programId.toBuffer(), mint_spl.toBuffer()], programId);
        console.log(`Pda : ${pda.toString()}`)



        let accounts: CreateMetadataAccountV3InstructionAccounts = {
            metadata: metadataPda,
            mint,
            mintAuthority: signer,
            payer: signer
        }

        let data: DataV2Args = {
            name: "Kavyam Token",
            symbol: "KYM",
            uri: "",
            sellerFeeBasisPoints: 500,
            creators: null,
            collection: null,
            uses: null
        }

        let args: CreateMetadataAccountV3InstructionArgs = {
            data,
            isMutable: true,
            collectionDetails: null
        }

        let tx = createMetadataAccountV3(
            umi,
            {
                ...accounts,
                ...args
            }
        )
        const accountInfo = await connection.getAccountInfo(pda);
        if (accountInfo) {
        console.log("Metadata account already exists:", metadataPda.toString());
        return;
        }

        // let result = await tx.sendAndConfirm(umi);
        // console.log(bs58.encode(result.signature));
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();

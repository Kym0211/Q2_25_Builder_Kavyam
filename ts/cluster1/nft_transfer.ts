import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, signerIdentity, generateSigner, percentAmount, publicKey } from "@metaplex-foundation/umi"
import { createNft, mplTokenMetadata, TokenStandard, transferV1 } from "@metaplex-foundation/mpl-token-metadata";

import wallet from "../cluster1/wallet/wallet.json"
import base58 from "bs58";

const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata());


(async () => {
    const mintAddress = publicKey("68RnsUU5ctqem2HaGpG1MSqHF6Ps6gEPeC7ZU2XWMLkR");
    const recipientAddress = publicKey("D9v5KH8Y9WLREt87nWALhstX2j5j1J5EMAu37XLVLoVm");
    try {
        const transferResult = await transferV1(umi, {
            mint: mintAddress,
            authority: myKeypairSigner,
            tokenOwner: myKeypairSigner.publicKey,
            destinationOwner: recipientAddress,
            tokenStandard: TokenStandard.NonFungible,
        }).sendAndConfirm(umi);

        console.log(`NFT transferred succesfully!`);
        console.log(`Transaction Signature: ${base58.encode(transferResult.signature)}`);
        console.log(`Explorer Link: https://explorer.solana.com/tx/${base58.encode(transferResult.signature)}?cluster=devnet`);
    } catch (error) {
        console.log(`Transfer failed: ${error}`)
    }

})();
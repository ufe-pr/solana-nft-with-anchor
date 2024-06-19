import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaNftWithAnchor } from "../target/types/solana_nft_with_anchor";
import { PublicKey, Umi, publicKey } from "@metaplex-foundation/umi";

import { walletAdapterIdentity } from "@metaplex-foundation/umi-signer-wallet-adapters";
import {
  MPL_TOKEN_METADATA_PROGRAM_ID,
  findMasterEditionPda,
  findMetadataPda,
  mplTokenMetadata,
} from "@metaplex-foundation/mpl-token-metadata";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
} from "@solana/spl-token";
import { Wallet } from "@coral-xyz/anchor/dist/cjs/provider";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";

describe("Mint NFT Program", () => {
  const metadata = {
    name: "Kobeni",
    symbol: "kBN",
    uri: "https://raw.githubusercontent.com/687c/solana-nft-native-client/main/metadata.json",
  };

  let ata: anchor.web3.PublicKey;
  let metadataAccount: PublicKey;
  let masterEdition: PublicKey;
  let umi: Umi;
  let mint: anchor.web3.Keypair;
  let program: Program<SolanaNftWithAnchor>;
  let signer: Wallet;
  let provider: anchor.AnchorProvider;

  before(async () => {
    program = anchor.workspace
      .SolanaNftWithAnchor as Program<SolanaNftWithAnchor>;
    provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    signer = provider.wallet;

    umi = createUmi(provider.connection.rpcEndpoint);
    umi.use(walletAdapterIdentity(signer)).use(mplTokenMetadata());

    mint = anchor.web3.Keypair.generate();
    ata = await getAssociatedTokenAddress(mint.publicKey, signer.publicKey);
    metadataAccount = findMetadataPda(umi, {
      mint: publicKey(mint.publicKey),
    })[0];
    masterEdition = findMasterEditionPda(umi, {
      mint: publicKey(mint.publicKey),
    })[0];
  });

  it("mints the nft", async () => {
    const tx = await program.methods
      .initNft(metadata.name, metadata.symbol, metadata.uri)
      .accounts({
        associatedTokenAccount: ata,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        masterEditionAccount: masterEdition,
        metadataAccount: metadataAccount,
        metadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
        mint: mint.publicKey,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        signer: signer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([mint])
      .rpc();

    console.log("tx", tx);
  });
});

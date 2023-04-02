import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { GifList } from "../target/types/gif_list";
import { GifListAdmin } from "../target/types/gif_list_admin";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const user = anchor.web3.Keypair.generate();

const addFunds = async (wallet: anchor.web3.Keypair, amount: number) => {
  const tx = await provider.connection.requestAirdrop(wallet.publicKey, amount);
  const latestHash = await provider.connection.getLatestBlockhash();
  await provider.connection.confirmTransaction({
    blockhash: latestHash.blockhash,
    lastValidBlockHeight: latestHash.lastValidBlockHeight,
    signature: tx
  });
};

const program = anchor.workspace.GifList as Program<GifList>;
const [allowedGifsPda] = anchor.web3.PublicKey.findProgramAddressSync([
  anchor.utils.bytes.utf8.encode('allowed_gifs'),
  user.publicKey.toBuffer(),
], program.programId);

const adminProgram = anchor.workspace.GifListAdmin as Program<GifListAdmin>;
const [gifListPda] = anchor.web3.PublicKey.findProgramAddressSync([
  anchor.utils.bytes.utf8.encode('gif_list'),
  provider.wallet.publicKey.toBuffer(),
], adminProgram.programId);
const [conditionsPda] = anchor.web3.PublicKey.findProgramAddressSync([
  anchor.utils.bytes.utf8.encode('conditions'),
  provider.wallet.publicKey.toBuffer(),
], adminProgram.programId);

describe("gif-list-admin", () => {
  it("initializes", async () => {
    const tx = await adminProgram.methods
      .initialize(new anchor.BN(0))
      .accounts({
        authority: provider.wallet.publicKey,
        gifList: gifListPda,
        conditions: conditionsPda
      })
      .rpc();
    console.log('Your transaction signature', tx);

    const gifList = await adminProgram.account.gifList.fetch(gifListPda);
    console.log('Gif list', gifList);
  });

  it('can add a new gif', async () => {
    const tx = await adminProgram.methods
      .addGif("https://media.giphy.com/media/MDJ9IbxxvDUQM/giphy.gif")
      .accounts({
        authority: provider.wallet.publicKey,
        gifList: gifListPda
      })
      .rpc();
    console.log('Your transaction signature', tx);
    
    const gifList = await adminProgram.account.gifList.fetch(gifListPda);
    console.log('Gif list', gifList);
  });
});

describe("gif-list", () => {
  it("initializes", async () => {
    await addFunds(user, anchor.web3.LAMPORTS_PER_SOL * 1);
    console.log('ADDED FUNDS');
    const tx = await program.methods
      .initialize()
      .accounts({
        authority: user.publicKey,
        allowedGifs: allowedGifsPda
      })
      .signers([user])
      .rpc();
    // await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [user]);
    console.log("Your transaction signature", tx);

    const allowedGifs = await program.account.allowedGifs.fetch(allowedGifsPda);
    const gifList = await adminProgram.account.gifList.fetch(gifListPda);
    console.log('Allowed gifs: ', allowedGifs);
    console.log('Gif list: ', gifList);
    
  });
});
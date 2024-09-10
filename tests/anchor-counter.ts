import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";
import { AnchorCounter } from "../target/types/anchor_counter";
import { PublicKey } from "@solana/web3.js";

describe("Anchor counter", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorCounter as Program<AnchorCounter>;
  const counterKeypair = anchor.web3.Keypair.generate();
  let counterPubkey: PublicKey;

  before(async () => {
    counterPubkey = counterKeypair.publicKey;
  });

  it("initializes the counter", async () => {
    try {
      await program.methods
        .initialize()
        .accounts({
          counter: counterPubkey,
          user: provider.wallet.publicKey
        })
        .signers([counterKeypair])
        .rpc();

      const account = await program.account.counter.fetch(counterPubkey);
      expect(account.count.toNumber()).to.equal(0);
    } catch (error) {
      console.error("Initialization failed:", error);
      throw error;
    }
  });

  it("increments the count", async () => {
    try {
      await program.methods
        .increment()
        .accounts({
          counter: counterPubkey,
          user: provider.wallet.publicKey,
        })
        .rpc();

      const account = await program.account.counter.fetch(counterPubkey);
      expect(account.count.toNumber()).to.equal(1);
    } catch (error) {
      console.error("Increment failed:", error);
      throw error;
    }
  });
});

import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { AnchorTestProject } from "../target/types/anchor_test_project";

describe("anchor-test-project", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorTestProject as Program<AnchorTestProject>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});

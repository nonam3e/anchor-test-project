const assert = require("assert");
const anchor = require("@project-serum/anchor");
const { SystemProgram } = anchor.web3;

describe("anchor-test-project", () => {
  /* create and set a Provider */
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  it("It initializes accounts", async () => {
      const dataAccount = anchor.web3.Keypair.generate();
      const crowdAccount = anchor.web3.Keypair.generate();
      const program = anchor.workspace.AnchorTestProject;

      await program.rpc.initialize({
          accounts: {
              crowdAccount: crowdAccount.publicKey,
              adminAccount: provider.wallet.publicKey,
              systemProgram: SystemProgram.programId,
              dataAccount: dataAccount.publicKey,
          },
          signers: [crowdAccount,dataAccount],
      });

    const account = await program.account.dataAccount.fetch(dataAccount.publicKey);
    const check = await program.account.crowdAccount.fetch(crowdAccount.publicKey)
    console.log('All data: ', account.dataList);
    assert.ok(account.dataList[0] === ("Success"));
    assert.ok(check.ownerId.toString() === provider.wallet.publicKey.toString())
    assert.ok(account.ownerId.toString() === crowdAccount.publicKey.toString());
  });

  // it("Updates a previously created account", async () => {
  //   const baseAccount = _baseAccount;
  //
  //   await program.rpc.update("Some new data", {
  //     accounts: {
  //       baseAccount: baseAccount.publicKey,
  //     },
  //   });
  //
  //   const account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  //   console.log('Updated data: ', account.data)
  //   assert.ok(account.data === "Some new data");
  //   console.log('all account data:', account)
  //   console.log('All data: ', account.dataList);
  //   assert.ok(account.dataList.length === 2);
  // });
});

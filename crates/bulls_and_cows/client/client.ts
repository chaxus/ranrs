const program = pg.program;
const seeds = Buffer.from("guessing pda");
const guessingPdaPubkey = anchor.web3.PublicKey.findProgramAddressSync(
  [seeds],
  program.programId
);

async function initialize() {
  try {
    const initializeTx = await program.methods
      .initialize()
      .accounts({
        guessingAccount: guessingPdaPubkey[0],
        payer: pg.wallet.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();

    console.log(
      "Initialize successfully!\n Your transaction signature is:",
      initializeTx
    );
  } catch (errors: any) {
    console.log(errors);
  }
}

async function guessing(number: number) {
  try {
    const guessingTx = await program.methods
      .guess(number)
      .accounts({
        guessingAccount: guessingPdaPubkey[0],
        payer: pg.wallet.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();
    console.log("Congratulation you're right!");
  } catch (errors: any) {
    console.log(errors.error.errorMessage);
  }
}

initialize();
// guessing(0);
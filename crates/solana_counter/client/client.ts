import * as web3 from "@solana/web3.js";
import https from 'node:https'
// Manually initialize variables that are automatically defined in Playground
const PROGRAM_ID = new web3.PublicKey(
  "3wGUG3qnLtCZFg3ukqeQXNhVYjrr3Jai4RnzEDyqjphc",
);

const connection = new web3.Connection(
  "https://api.devnet.solana.com",
  "confirmed",
);
const wallet = { keypair: web3.Keypair.generate() };

// Client
console.log("My address:", wallet.keypair.publicKey.toString());
connection.getBalance(wallet.keypair.publicKey).then(balance => {
  console.log(`My balance: ${balance / web3.LAMPORTS_PER_SOL} SOL`);
})

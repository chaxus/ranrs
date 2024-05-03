import * as borsh from "borsh";
import assert from "assert";
import * as web3 from "@solana/web3.js";
// Manually initialize variables that are automatically defined in Playground
// test environment
const PROGRAM_ID = new web3.PublicKey(
  "3wGUG3qnLtCZFg3ukqeQXNhVYjrr3Jai4RnzEDyqjphc",
);
const connection = new web3.Connection(
  "https://api.devnet.solana.com",
  "confirmed",
);
const wallet = { keypair: web3.Keypair.generate() };

/**
 * CounterAccount 对象
 */
class CounterAccount {
  count = 0;
  constructor(fields: { count: number } | undefined = undefined) {
    if (fields) {
      this.count = fields.count;
    }
  }
}

/**
 * CounterAccount 对象 schema 定义
 */
const CounterAccountSchema = new Map([
  [CounterAccount, { kind: "struct", fields: [["count", "u32"]] }],
]);

/**
 * 账户空间大小
 */
const GREETING_SIZE = borsh.serialize(
  CounterAccountSchema,
  new CounterAccount(),
).length;

describe("Test", () => {
  it("greet", async () => {
    // 创建 keypair
    const counterAccountKp = new web3.Keypair();
    console.log(`counterAccountKp.publickey : ${counterAccountKp.publicKey}`);
    const lamports =
      await connection.getMinimumBalanceForRentExemption(GREETING_SIZE);

    // 创建生成对应数据账户的指令
    const createGreetingAccountIx = web3.SystemProgram.createAccount({
      fromPubkey: wallet.keypair.publicKey,
      lamports,
      newAccountPubkey: counterAccountKp.publicKey,
      programId: PROGRAM_ID,
      space: GREETING_SIZE,
    });

    // 调用程序,计数器累加
    const greetIx = new web3.TransactionInstruction({
      keys: [
        {
          pubkey: counterAccountKp.publicKey,
          isSigner: false,
          isWritable: true,
        },
      ],
      programId: PROGRAM_ID,
    });

    // 创建交易，包含如上2个指令
    const tx = new web3.Transaction();
    tx.add(createGreetingAccountIx, greetIx);

    // 发起交易，获取交易哈希
    const txHash = await web3.sendAndConfirmTransaction(connection, tx, [
      wallet.keypair,
      counterAccountKp,
    ]);
    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);

    // 获取指定数据账户的信息
    const counterAccountOnSolana = await connection.getAccountInfo(
      counterAccountKp.publicKey,
    );

    // 反序列化
    const deserializedAccountData = borsh.deserialize(
      CounterAccountSchema,
      CounterAccount,
      counterAccountOnSolana.data,
    );

    // 判断当前计数器是否累加
    assert.equal(deserializedAccountData.count, 1);
  });
});

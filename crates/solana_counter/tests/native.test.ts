import * as borsh from "borsh"
import assert from "assert"
import {
  Connection,
  PublicKey,
  Keypair,
  SystemProgram,
  TransactionInstruction,
  Transaction,
  sendAndConfirmTransaction,
  clusterApiUrl
} from "@solana/web3.js"
// Manually initialize variables that are automatically defined in Playground
// test environment
const PROGRAM_ID = new PublicKey("3wGUG3qnLtCZFg3ukqeQXNhVYjrr3Jai4RnzEDyqjphc")
const connection = new Connection(clusterApiUrl("devnet"), "confirmed")
const wallet = { keypair: Keypair.generate() }
/**
 * CounterAccount 对象
 */
class CounterAccount {
  count = 0
  constructor(fields: { count: number } | undefined = undefined) {
    if (fields) {
      this.count = fields.count
    }
  }
}

/**
 * CounterAccount 对象 schema 定义
 */
const CounterAccountSchema = new Map([[CounterAccount, { kind: "struct", fields: [["count", "u32"]] }]])

/**
 * 账户空间大小
 */
const GREETING_SIZE = borsh.serialize(CounterAccountSchema, new CounterAccount()).length

describe("Test", () => {
  it("greet", async () => {
    // 创建 keypair
    // 创建了一个新的 Solana Keypair (counterAccountKp) 用于存储计数器的状态。
    const counterAccountKp = new Keypair()
    console.log(`counterAccountKp.publickey : ${counterAccountKp.publicKey}`)
    try {
      console.log('GREETING_SIZE',GREETING_SIZE);
      const lamports = await connection.getMinimumBalanceForRentExemption(GREETING_SIZE)
      // 创建生成对应数据账户的指令
      // 使用 Solana API 获取在链上创建相应账户所需的最小 lamports，即Solana 链上存储该账户所要支付的最小押金rent。
      const createGreetingAccountIx = SystemProgram.createAccount({
        fromPubkey: wallet.keypair.publicKey,
        lamports,
        newAccountPubkey: counterAccountKp.publicKey,
        programId: PROGRAM_ID,
        space: GREETING_SIZE,
      })
      // 调用程序,计数器累加
      const greetIx = new TransactionInstruction({
        keys: [
          {
            pubkey: counterAccountKp.publicKey,
            isSigner: false,
            isWritable: true,
          },
        ],
        programId: PROGRAM_ID,
      })

      // 创建交易，包含如上2个指令
      const tx = new Transaction()
      tx.add(createGreetingAccountIx, greetIx)
      // 发起交易，获取交易哈希
      const txHash = await sendAndConfirmTransaction(connection, tx, [wallet.keypair, counterAccountKp])
      console.log(`Use 'solana confirm -v ${txHash}' to see the logs`)

      // 获取指定数据账户的信息
      const counterAccountOnSolana = await connection.getAccountInfo(counterAccountKp.publicKey)

      // 反序列化
      const deserializedAccountData = borsh.deserialize(CounterAccountSchema, CounterAccount, counterAccountOnSolana.data)

      // 判断当前计数器是否累加
      assert.equal(deserializedAccountData.count, 1)
    } catch (error) {
      console.log('error--->', error);
    }
  })
})

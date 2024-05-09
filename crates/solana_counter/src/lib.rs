// 该结构体中定义了u32类型的count属性，在我们每次发起交易指令时，它都会+1操作。
// 因为该值的存储和传输都是使用的字节码，要把字节码转为Rust 类型，我们还需要（反）序列化操作，这里需要引入borsh库。
use borsh::{BorshDeserialize, BorshSerialize};
// 要使用 Rust 编写 Solana 程序，需要用到 Solana 程序的标准库 solana_program。
// 该标准库包含我们将用于开发 Solana 程序的模块和宏。
// 如果您想深入了解solana_program  crate，请查看solana_program crate文档
// https://docs.rs/solana-program/latest/solana_program/index.html
use solana_program::{
    account_info::{next_account_info, AccountInfo}, // account_info 模块中的一个结构体，允许我们访问帐户信息。
    entrypoint, // 声明程序入口点的宏，类似于 Rust 中的 main 函数。
    entrypoint::ProgramResult, // ProgramResult: entrypoint 模块中的返回值类型。
    msg, // 一个允许我们将消息打印到程序日志的宏，类似于 Rust 中的 println宏。
    program_error::ProgramError,
    pubkey::Pubkey, // pubkey 模块中的一个结构体，允许我们将地址作为公钥访问。
};

// Solana 程序需要单个入口点来处理程序指令。入口点是使用entrypoint!声明的宏。
// 定义程序入口点函数
entrypoint!(process_instruction);

// Solana 程序帐户仅存储处理指令的逻辑。
// 这意味着程序帐户是“只读”和“无状态”的。程序处理指令所需的“状态”（数据集）存储在数据帐户中（与程序帐户分开）。

/// 定义数据账户的结构
#[derive(BorshSerialize, BorshDeserialize, Debug)]
// 我们的 Solana 程序要实现计数器的累加，
// 那就必须先定义该数据是以何种形式存储在 Solana 链上的，这里我们使用结构体CounterAccount，
// 之所以使用Account后缀，因为它是一个数据账户
pub struct CounterAccount {
    pub count: u32,
}

pub fn process_instruction(
    // 程序ID，即程序地址，当前的程序ID
    program_id: &Pubkey,
    // 该指令涉及到的账户集合
    accounts: &[AccountInfo],
    // 该指令的参数
    _instruction_data: &[u8],
) -> ProgramResult {
    // 为了处理指令，指令所需的数据帐户必须通过accounts参数显式传递到程序中。
    // 这里因为要对数据账户进行累加的操作，所以 accounts 包含了该数据账户，我们通过迭代器获取到该账户account。
    msg!("Hello World Rust program entrypoint");

    // 账户迭代器
    let accounts_iter = &mut accounts.iter();

    // 获取调用者账户
    let account = next_account_info(accounts_iter)?;
    // account数据账户是由该程序派生出来的账户，因此当前程序为它的owner所有者，
    // 并且只有所有者才可以对其进行写操作。所以我们在这里要进行账户权限的校验。
    // 验证调用者身份
    if account.owner != program_id {
        msg!("Counter account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }
    // 读取并写入新值

    // 这行代码的目的是从 Solana 数据账户中反序列化出 CounterAccount 结构体的实例。

    // &account.data：获取账户的数据字段的引用。在 Solana 中，
    // 账户的数据字段data存储着与账户关联的实际数据，对于程序账户而言，它是程序的二进制内容，对于数据账户而言，它就是存储的数据。

    // borrow()：使用该方法获取data数据字段的可借用引用。并通过&account.data.borrow()方式得到账户数据字段的引用。

    // CounterAccount::try_from_slice(...)：调用try_from_slice方法，它是BorshDeserializetrait 的一个方法，用于从字节序列中反序列化出一个结构体的实例。
    // 这里CounterAccount实现了BorshDeserialize，所以可以使用这个方法。

    // ?：是一个错误处理操作符，如果try_from_slice返回错误，整个表达式将提前返回，将错误传播给调用方。
    let mut counter = CounterAccount::try_from_slice(&account.data.borrow())?;
    // 通过如上方式，我们获取了CounterAccount数据账户进行了反序列化，并获取到它的可变借用。
    // 接下来我们就可以对该数据账户进行修改：
    // 首先对CounterAccount结构体中的count字段进行递增操作。
    counter.count += 1;
    // &mut *account.data.borrow_mut()：通过borrow_mut()方法获取账户数据字段的可变引用，
    // 然后使用*解引用操作符获取该data字段的值，并通过&mut将其转换为可变引用。

    // serialize函数方法，它是BorshSerialize trait 的一个方法，用于将结构体序列化为字节数组。 
    // 通过如上的方式，将CounterAccount结构体中的修改后的值递增，并将更新后的结构体序列化为字节数组，
    // 然后写入 Solana 账户的可变数据字段中。实现了在 Solana 程序中对计数器值进行更新和存储。
    counter.serialize(&mut *account.data.borrow_mut())?;

    Ok(())
}

// 总的来说，在这个 Solana 实现的程序中，我们要自己实现（反）序列化、指定程序入口点、账号安全校验等操作，
// 这对于程序的开发是必须的，但也是繁琐的，如果使用 Anchor 框架，就能把我们从这些重复的劳动中解脱出来，更加专注于程序本身的业务逻辑，
// 在后续的章节，我们会专门介绍 Solana 的 Anchor 开发框架~
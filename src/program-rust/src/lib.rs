use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct DepositAccount {
    pub deposits:u32,
    pub plan:u32,
    pub percent:u32,
    pub amount:u32,
    pub profit:u32,
    pub start:u32,
    pub finish:u32,
    pub checkpoint:u32,
    // pub referrer:String,
    pub level1:u32,
    pub level2:u32,
    pub level3:u32,
    pub bonus:u32,
    pub totalbonus:u32,
}

pub mod utils;
// pub struct DepositAccount{
//     pub x:u32,
    
// }

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {

    pub const PREFIX: &str = "deposit";
    pub const MAX_NAME_LENGTH: usize = 32;
    pub const MAX_SYMBOL_LENGTH: usize = 10;
    pub const MAX_URI_LENGTH: usize = 200;
    pub const MAX_CREATOR_LIMIT: usize = 5;
    pub const MAX_CREATOR_LEN: usize = 32 + 1 + 1;

    pub const MAX_DATA_SIZE: usize = 4
    + MAX_NAME_LENGTH
    + 4
    + MAX_SYMBOL_LENGTH
    + 4
    + MAX_URI_LENGTH
    + 2
    + 1
    + 4
    + MAX_CREATOR_LIMIT * MAX_CREATOR_LEN;
    pub const MAX_METADATA_LEN: usize = 
1 //key 
+ 32 // update auth pubkey
+ 32 // mint pubkey
+ MAX_DATA_SIZE 
+ 1 // primary sale
+ 1 // mutable
+ 9 // nonce (pretty sure this only needs to be 2)
+ 34 // collection
+ 18 // uses
+ 2 // token standard
+ 118; // Padding

    msg!("Hello World program entrypoint");
    
    // Iterating accounts is safer than indexing
    let accounts_iter = &mut accounts.iter();

    // initializing accounts
    let deposit_account = next_account_info(accounts_iter)?;
    let rent_info = next_account_info(accounts_iter)?;
    let system_account_info = next_account_info(accounts_iter)?;
    let payer_account_info = next_account_info(accounts_iter)?;
    let ref1account = next_account_info(accounts_iter)?;
    let ref2account = next_account_info(accounts_iter)?;
    let ref3account = next_account_info(accounts_iter)?;

    let metadata_seeds = &[
        PREFIX.as_bytes(),
        program_id.as_ref(),
    ];

    let (metadata_key, metadata_bump_seed) =
        Pubkey::find_program_address(metadata_seeds, program_id);

    let metadata_authority_signer_seeds = &[
        PREFIX.as_bytes(),
        &payer_account_info.key.as_ref(),
        &[metadata_bump_seed],
    ];

    utils::create_or_allocate_account_raw(
        *program_id,
        deposit_account,
        rent_info,
        system_account_info,
        payer_account_info,
        MAX_METADATA_LEN,
        metadata_authority_signer_seeds,
    )?;

    let _invest_min_amount:u32 = 1; 
    let _project_fee:u32 = 100;
    let _percent_step:u32 = 5;
    let percents_divider:u32 = 1000;
    let referal_percents:[u32;3]=[50, 25, 5];


    // The account must be owned by the program in order to modify its data
    if deposit_account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // collecting data passed in the instruction 
    let mut deposit_account = DepositAccount::try_from_slice(&deposit_account.data.borrow())?;
    let mut ref1account = DepositAccount::try_from_slice(&ref1account.data.borrow())?;
    let mut ref2account = DepositAccount::try_from_slice(&ref2account.data.borrow())?;
    let mut ref3account = DepositAccount::try_from_slice(&ref3account.data.borrow())?;

    // collecting instruction data
    let amount : u32 = From::from(_instruction_data[0]);
    let plan :u32 = From::from(_instruction_data[1]);

    // deposit_account.deposits=255;
    // deposit_account.y=20;
    // deposit_account.z="123".to_string();
    // deposit_account.q=[1,2,3];
    deposit_account.deposits=1;
    deposit_account.plan=1;
    deposit_account.percent=2;
    deposit_account.amount=2;
    deposit_account.profit=2;
    deposit_account.start=2;
    deposit_account.finish=2;
    deposit_account.checkpoint=2;
    // deposit_account.referrer;
    deposit_account.level1=2;
    deposit_account.level2=2;
    deposit_account.level3=2;
    // deposit_account.plan.push(1);

    // increment level counters for referrers
    ref1account.level1=ref1account.level1+1;
    ref2account.level2=ref2account.level2+1;
    ref3account.level3=ref3account.level3+1;

    // add bonus to ref1account 
    let mut refamount =  referal_percents[0];
    refamount = refamount / percents_divider;
    ref1account.bonus = ref1account.bonus + refamount;
    ref1account.totalbonus = ref1account.totalbonus + refamount;

    // add bonus to ref2account
    let mut refamount =  referal_percents[1];
    refamount = refamount / percents_divider;
    ref2account.bonus = ref2account.bonus + refamount;
    ref2account.totalbonus = ref2account.totalbonus + refamount;

    // add bonus to ref3account
    let mut refamount =  referal_percents[2];
    refamount = refamount / percents_divider;
    ref3account.bonus = ref3account.bonus + refamount;
    ref3account.totalbonus = ref3account.totalbonus + refamount;

    deposit_account.deposits = amount;
    deposit_account.plan = plan;

    let (percent, profit, finish) = get_result(plan, amount);

    deposit_account.percent=percent;
    // deposit_account.amount=2;
    deposit_account.profit=profit;
    // deposit_account.start=2;
    deposit_account.finish=finish;

    

    // msg!("deposit_account.x {:?}", deposit_account.deposits);
    // msg!("deposit_account.y {:?}", deposit_account.plan);
    // msg!("deposit_account.z {:?}", deposit_account.percent);
    // msg!("deposit_account.q {:?}", deposit_account.amount);
    // msg!("deposit_account.profit {:?}", deposit_account.profit);
    // msg!("deposit_account.start {:?}", deposit_account.start);
    // msg!("deposit_account.finish {:?}", deposit_account.finish);
    // msg!("deposit_account.checkpoint {:?}", deposit_account.checkpoint);
    // msg!("deposit_account.levels {:?}", deposit_account.levels);
    // deposit_account.serialize(&mut &mut deposit_account.data.borrow_mut()[..])?;

    // msg!("Greeted {} time(s)!", deposit_account.counter);

    Ok(())
}
pub fn get_result(plan:u32, amount:u32) -> (u32, u32, u32) {
    let percent = 0; //getPercent(plan); safe_mul(percent); 
    //let plans = [];
    let mut x: Getdata = Getdata{
        percent: 0,
        profit:0,
        finish:0
    };

    if plan < 3 {
        x.profit = amount * percent;
        x.profit = x.profit / 1000;
        x.profit=x.profit * 3;
        } else if plan < 6 {
            for i in 1 .. 3 {
                 let mut adder = amount + x.profit;
                adder = adder * percent;
                adder = adder * 1000;
                x.profit = x.profit + adder;
                    }
                }
                x.finish = 1234;
                return (x.percent, x.profit, x.finish);
}
pub struct Getdata {
    percent : u32,
    profit: u32,
    finish: u32 
}

// // Sanity tests
// #[cfg(test)]
// mod test {
//     use super::*;
//     use solana_program::clock::Epoch;
//     use std::mem;

//     #[test]
//     fn test_sanity() {
//         let program_id = Pubkey::default();
//         let key = Pubkey::default();
//         let mut lamports = 0;
//         let mut data = vec![0; mem::size_of::<u32>()];
//         let owner = Pubkey::default();
//         let account = AccountInfo::new(
//             &key,
//             false,
//             true,
//             &mut lamports,
//             &mut data,
//             &owner,
//             false,
//             Epoch::default(),
//         );
//         let instruction_data: Vec<u8> = Vec::new();

//         let accounts = vec![account];

//         assert_eq!(
//             DepositAccount::try_from_slice(&accounts[0].data.borrow())
//                 .unwrap()
//                 .counter,
//             0
//         );
//         process_instruction(&program_id, &accounts, &instruction_data).unwrap();
//         assert_eq!(
//             DepositAccount::try_from_slice(&accounts[0].data.borrow())
//                 .unwrap()
//                 .counter,
//             1
//         );
//         process_instruction(&program_id, &accounts, &instruction_data).unwrap();
//         assert_eq!(
//             DepositAccount::try_from_slice(&accounts[0].data.borrow())
//                 .unwrap()
//                 .counter,
//             2
//         );
//     }
// }

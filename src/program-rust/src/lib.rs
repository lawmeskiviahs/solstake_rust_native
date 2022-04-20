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
pub struct GreetingAccount {
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
// pub struct GreetingAccount{
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
    msg!("Hello World program entrypoint");

    // Iterating accounts is safer than indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Increment and store the number of times the account has been greeted
    let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;
    // greeting_account.deposits=255;
    // greeting_account.y=20;
    // greeting_account.z="123".to_string();
    // greeting_account.q=[1,2,3];
    greeting_account.deposits=1;
    greeting_account.plan=1;
    greeting_account.percent=2;
    greeting_account.amount=2;
    greeting_account.profit=2;
    greeting_account.start=2;
    greeting_account.finish=2;
    greeting_account.checkpoint=2;
    // greeting_account.referrer;
    greeting_account.level1=2;
    greeting_account.level2=2;
    greeting_account.level3=2;
    // greeting_account.plan.push(1);
    msg!("greeting_account.x {:?}", greeting_account.deposits);
    // msg!("greeting_account.y {:?}", greeting_account.plan);
    // msg!("greeting_account.z {:?}", greeting_account.percent);
    // msg!("greeting_account.q {:?}", greeting_account.amount);
    // msg!("greeting_account.profit {:?}", greeting_account.profit);
    // msg!("greeting_account.start {:?}", greeting_account.start);
    // msg!("greeting_account.finish {:?}", greeting_account.finish);
    // msg!("greeting_account.checkpoint {:?}", greeting_account.checkpoint);
    // msg!("greeting_account.levels {:?}", greeting_account.levels);
    greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    // msg!("Greeted {} time(s)!", greeting_account.counter);

    Ok(())
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
//             GreetingAccount::try_from_slice(&accounts[0].data.borrow())
//                 .unwrap()
//                 .counter,
//             0
//         );
//         process_instruction(&program_id, &accounts, &instruction_data).unwrap();
//         assert_eq!(
//             GreetingAccount::try_from_slice(&accounts[0].data.borrow())
//                 .unwrap()
//                 .counter,
//             1
//         );
//         process_instruction(&program_id, &accounts, &instruction_data).unwrap();
//         assert_eq!(
//             GreetingAccount::try_from_slice(&accounts[0].data.borrow())
//                 .unwrap()
//                 .counter,
//             2
//         );
//     }
// }

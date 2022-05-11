use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint::ProgramResult,
  program_error::ProgramError,
  msg,
  pubkey::Pubkey,
  program_pack::Pack,
  sysvar::{rent::Rent, Sysvar, clock::Clock},
  program::invoke,
  program::invoke_signed,
  system_instruction::create_account,
};
use spl_token::instruction::transfer;

use crate::{instruction::LockInstruction, state::LockState};

pub struct Processor;
impl Processor {
  pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let instruction = LockInstruction::unpack(instruction_data)?;

    match instruction {
      LockInstruction::CreateAccounts{bump} => {
        msg!("Instruction: CreateAccounts");
        Self::process_create_accounts(accounts, program_id, bump)
      },
      
      LockInstruction::LockTokens { amount } => {
        msg!("Instrution: LockTokens");
        Self::process_lock_tokens(accounts, program_id, amount)
      },
      LockInstruction::UnlockTokens {} => {
        msg!("Instrution: UnlockTokens");
        Self::process_unlock_tokens(accounts, program_id)
      }
    }
  }

  fn process_create_accounts (
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    bump: u8,
  ) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let user = next_account_info(account_iter)?;
    let lock_state_account = next_account_info(account_iter)?;
    let rent_account = next_account_info(account_iter)?;
    let system_program = next_account_info(account_iter)?;
    let lock_state_token = next_account_info(account_iter)?;

    if !user.is_signer {
      msg!("Given user is not a signer");
      return Err(ProgramError::InvalidArgument)
    }

    let is_initialized = lock_state_account.try_borrow_data()?[0] == 1;
    if is_initialized {
      msg!("lock state account has already been initialized");
      return Err(ProgramError::InvalidArgument)
    } 

    let lock_state_token_data = spl_token::state::Account::unpack_from_slice(
      &lock_state_token.data.borrow()
    )?;
    if lock_state_token_data.owner != *lock_state_account.key {
      msg!("lock state token account is not owned by the lock state account");
      return Err(ProgramError:: InvalidArgument)
    }

    let bump_slice = &[bump];
    let lock_state_seed = &[
      user.key.as_ref(),
      bump_slice,
    ][..];
    let derived_lock_state = Pubkey::create_program_address(
      lock_state_seed,
      program_id,
    )?;
    if derived_lock_state != *lock_state_account.key {
      msg!("Invalid lock state key");
      return Err(ProgramError::InvalidArgument)
    }

    if solana_program::system_program::id() != *system_program.key {
      msg!("Invalid system program provided");
      return Err(ProgramError::InvalidArgument)
    }
    if solana_program::sysvar::rent::id() != *rent_account.key {
      msg!("Invalid rent program provided");
      return Err(ProgramError::InvalidArgument)
    }

    let size = LockState::LEN;
    let rent = Rent::from_account_info(rent_account)?;
    let rent_lamports = rent.minimum_balance(size);
    let create_account_ix = create_account(
      &user.key,
      &lock_state_account.key,
      rent_lamports,
      size as u64,
      &program_id,
    );
    invoke_signed(
      &create_account_ix,
      &[
        user.clone(),
        lock_state_account.clone()
      ],
      &[&lock_state_seed]
    )?;

    let mut lock_state = LockState::unpack_from_slice(&lock_state_account.data.borrow())?;
    lock_state.is_initialized = true;
    lock_state.user = *user.key;
    lock_state.bump = bump;
    lock_state.unlock_time = 0;
    lock_state.locked_tokens = 0;
    lock_state.locked_token_account = *lock_state_token.key;
    LockState::pack(lock_state, &mut lock_state_account.data.borrow_mut())?;
    
    Ok(())
  }

  fn process_lock_tokens(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    amount: u64,
  ) -> ProgramResult {
    //Your code here!
  
    Ok(())
  }

  fn process_unlock_tokens(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
  ) -> ProgramResult {
    
    Ok(())
  }

}
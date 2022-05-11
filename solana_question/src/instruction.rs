use std::convert::TryInto;
use solana_program::{ 
  msg,
  program_error::ProgramError
};

pub enum LockInstruction{

  CreateAccounts {
    bump: u8,
  },
  LockTokens {
    amount: u64,
  },

  UnlockTokens {
  },
  
}

impl LockInstruction {
  pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
    let (tag, rest) = input.split_first().ok_or(ProgramError::InvalidArgument)?;
    msg!("rest {:?}", rest);
    Ok(match tag {
      0 => Self::CreateAccounts {
        bump: rest[0],
      }, 
      
      1 => Self::LockTokens {
        amount: rest.
          get(..8)
          .and_then(|slice| slice.try_into().ok())
          .map(u64::from_le_bytes)
          .ok_or(ProgramError::InvalidArgument)?,
      }, 
      2 => Self::UnlockTokens {

      },
      _ => return Err(ProgramError::InvalidArgument)
    })
  }
}
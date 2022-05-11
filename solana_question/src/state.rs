use solana_program::{
  program_pack::{IsInitialized, Pack, Sealed},
  pubkey::Pubkey,
  program_error::ProgramError,
};
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

#[derive(Debug)]
pub struct LockState {
  pub is_initialized: bool,
  pub user: Pubkey,
  pub bump: u8,
  pub locked_tokens: u64,
  pub unlock_time: i64,
  pub locked_token_account: Pubkey,
}

impl IsInitialized for LockState {
  fn is_initialized(&self) -> bool {
    self.is_initialized
  }
}

impl Sealed for LockState {}

impl Pack for LockState {
  const LEN: usize = 82;

  fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
    let src = array_ref![src, 0, LockState::LEN];
    let (
      is_initialized,
      user,
      bump,
      locked_tokens,
      unlock_time,
      locked_token_account,
    ) = array_refs![src, 1, 32, 1, 8, 8, 32];

    let is_initialized = match is_initialized {
      [0] => false,
      [1] => true,
      _ => return Err(ProgramError::InvalidAccountData),
    };

    Ok(LockState {
      is_initialized,
      user: Pubkey::new_from_array(*user),
      bump: bump[0],
      locked_tokens: u64::from_le_bytes(*locked_tokens),
      unlock_time: i64::from_le_bytes(*unlock_time),
      locked_token_account: Pubkey::new_from_array(*locked_token_account),
    })
  }

  fn pack_into_slice(&self, dst: &mut [u8]) {
    let dst = array_mut_ref![dst, 0, LockState::LEN];
    let (
      is_initialized_dst,
      user_dst,
      bump_dst,
      locked_tokens_dst,
      unlock_time_dst,
      locked_token_account_dst,
    ) = mut_array_refs![dst, 1, 32, 1, 8, 8, 32];

    let LockState {
      is_initialized,
      user,
      bump,
      locked_tokens,
      unlock_time,
      locked_token_account,
    } = self;

    is_initialized_dst[0] = *is_initialized as u8;
    user_dst.copy_from_slice(user.as_ref());
    bump_dst[0] = *bump;
    *locked_tokens_dst = locked_tokens.to_le_bytes();
    *unlock_time_dst = unlock_time.to_le_bytes();
    locked_token_account_dst.copy_from_slice(locked_token_account.as_ref());
  }
}
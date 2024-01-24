use borsh::BorshDeserialize;
use borsh_derive::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct UpdateArgs {
    pub value: u32,
}

pub enum CounterInstructions {
    Increment(u32),
    Decrement(u32),
    Update(UpdateArgs),
    Reset,
}

//the unpack function here related to the counter
impl CounterInstructions {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match variant {}
                0 => Self::Increment(u32::from_le_bytes(
            <[u8; 4]>::try_from(&rest[..4]).map_err(|_| ProgramError::InvalidInstructionData)?,
        )),
        1 => Self::Decrement(u32::from_le_bytes(
            <[u8; 4]>::try_from(&rest[..4]).map_err(|_| ProgramError::InvalidInstructionData)?,
        )),
        2 => Self::Update(UpdateArgs::unpack(rest)?),
        3 => Self::Reset,
        _ => return Err(ProgramError::InvalidInstructionData),
    })
    }
}

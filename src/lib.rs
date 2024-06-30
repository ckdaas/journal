use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

// Define the structure for a journal entry
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct JournalEntry {
    pub id: u64,
    pub content: String,
}

// Define the structure for the program state
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct JournalState {
    pub entries: Vec<JournalEntry>,
    pub entry_counter: u64,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    if account.owner != program_id {
        return Err(solana_program::program_error::ProgramError::IncorrectProgramId);
    }

    let mut journal_state = JournalState::try_from_slice(&account.data.borrow())?;

    let instruction = instruction_data[0];
    match instruction {
        0 => {
            // Create
            let entry = JournalEntry::try_from_slice(&instruction_data[1..])?;
            journal_state.entries.push(entry);
            journal_state.entry_counter += 1;
        }
        1 => {
            // Update
            let id = u64::try_from_slice(&instruction_data[1..9])?;
            let new_content = String::try_from_slice(&instruction_data[9..])?;
            if let Some(entry) = journal_state.entries.iter_mut().find(|e| e.id == id) {
                entry.content = new_content;
            }
        }
        2 => {
            // Delete
            let id = u64::try_from_slice(&instruction_data[1..])?;
            journal_state.entries.retain(|e| e.id != id);
        }
        _ => {
            msg!("Invalid instruction");
            return Err(solana_program::program_error::ProgramError::InvalidInstructionData);
        }
    }

    journal_state.serialize(&mut &mut account.data.borrow_mut()[..])?;

    Ok(())
}

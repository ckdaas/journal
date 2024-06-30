use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};

// Define the structures (same as in the program)
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct JournalEntry {
    pub id: u64,
    pub content: String,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct JournalState {
    pub entries: Vec<JournalEntry>,
    pub entry_counter: u64,
}

fn main() {
    let rpc_url = "http://localhost:8899".to_string();
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // Replace with your program ID
    let program_id = Pubkey::new_unique();

    // Create a new keypair for the journal account
    let journal_account = Keypair::new();

    // Create a new keypair for the payer
    let payer = Keypair::new();

    // Fund the payer account
    let airdrop_signature = client
        .request_airdrop(&payer.pubkey(), 1_000_000_000)
        .unwrap();
    client.confirm_transaction(&airdrop_signature).unwrap();

    // Fund the journal account
    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let transaction = Transaction::new_signed_with_payer(
        &[system_instruction::create_account(
            &payer.pubkey(),
            &journal_account.pubkey(),
            10000000, // lamports
            1000,     // space
            &program_id,
        )],
        Some(&payer.pubkey()),
        &[&payer, &journal_account],
        recent_blockhash,
    );
    client.send_and_confirm_transaction(&transaction).unwrap();

    // Create a new journal entry
    let new_entry = JournalEntry {
        id: 1,
        content: "First entry".to_string(),
    };
    let instruction_data = [0]
        .iter()
        .chain(new_entry.try_to_vec().unwrap().iter())
        .cloned()
        .collect::<Vec<u8>>();
    let instruction = Instruction::new_with_borsh(
        program_id,
        &instruction_data,
        vec![AccountMeta::new(journal_account.pubkey(), false)],
    );

    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
    client.send_and_confirm_transaction(&transaction).unwrap();

    println!("Journal entry created successfully!");
}

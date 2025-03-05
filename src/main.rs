use solana_sdk::system_program;
use solana_sdk::instruction::AccountMeta;
use rand::Rng;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::Instruction,
    message::Message,
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair, Signer},
    transaction::Transaction,
};
use std::io::{self, Write};
use std::str::FromStr;

fn main() {
    // Connect to the Solana devnet
    let rpc_url = "https://api.devnet.solana.com";
    let client =
        RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    // Load the payer's keypair
    let payer = read_keypair_file("payer-keypair.json")
        .expect("Failed to read payer keypair file");

    // Receiver's public key
    let receiver_pubkey_str = "3KwfmvwNvirudqezF8RXZ9mbeLj8xamFzjYurJytiEFy"; // Replace with your receiver's public key
    let receiver_pubkey =
        Pubkey::from_str(receiver_pubkey_str).expect("Failed to parse receiver public key");

    // Your deployed smart contract's Program ID
    let program_id = Pubkey::from_str("7NSTRzTTrnADkak3ADuQa2rx4YNnp2PUjEYsmyahProV")
        .expect("Failed to parse program ID");

    // Implement the guessing game
    println!("Welcome to the Guessing Game!");
    println!("Guess a number between 1 and 10:");

    // Generate a random number between 1 and 10
    let secret_number = rand::thread_rng().gen_range(1..=10);

    // For testing purposes, you might want to display the secret number
    // println!("(The secret number is: {})", secret_number);

    // Get the user's guess
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please input a valid number!");
            return;
        }
    };

    println!("You guessed: {}", guess);

    // Check the guess
    if guess == secret_number {
        println!("Congratulations! You guessed correctly. No lamports will be transferred.");
    } else {
        println!("Wrong guess! Transferring lamports to the receiver...");

        // Amount to transfer (e.g., 1,000,000 lamports = 0.001 SOL)
        let amount: u64 = 1_000_000;

        // Prepare instruction data: the amount in little-endian byte order
        let amount_le_bytes = amount.to_le_bytes();

        // Create the instruction to invoke the smart contract
        let instruction = Instruction::new_with_bytes(
            program_id,
            &amount_le_bytes,
            vec![
                // Accounts expected by the smart contract
                AccountMeta::new(payer.pubkey(), true), // Payer (signer)
                AccountMeta::new(receiver_pubkey, false), // Receiver
                AccountMeta::new_readonly(system_program::id(),false),
            ],
        );

        // Create a transaction
        let recent_blockhash = client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
        let message = Message::new(&[instruction], Some(&payer.pubkey()));
        let transaction = Transaction::new(&[&payer], message, recent_blockhash);

        // Send the transaction
        match client.send_and_confirm_transaction(&transaction) {
            Ok(signature) => {
                println!("Transaction successful!");
                println!("Signature: {}", signature);
            }
            Err(err) => {
                eprintln!("Transaction failed:");
                eprintln!("{}", err);
            }
        }
    }
}

use {
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program,
        clock::Epoch,
        account_info::AccountInfo
    },
    solana_program_test::*,
    solana_sdk::{signature::{Signer, Keypair}, transaction::Transaction, account::Account as SDKAccount},
    another_payer::processor
};

use bumpalo::{Bump};

use crate::processor::Processor;

#[tokio::test]
async fn test_escrow() {
    let bump = Bump::new();
    let my_keypair = Keypair::new();
    let my_keypair_account = SDKAccount::new(1000_0000_000_000, 500, &system_program::ID);
    AccountInfo::new(
        &my_keypair.pubkey(),
        false,
        true,
        bump.alloc(1000_0000_000_000),
        &mut [],
        &system_program::ID,
        false,
        Epoch::default(),
    );
    let program_id = Pubkey::new_unique();
    let mut pc = ProgramTest::new(
        "another-payer",
        program_id,
        processor!(Processor::process),
    );
    pc.add_account(my_keypair.pubkey(), my_keypair_account);

    let (mut banks_client, _payer, recent_blockhash) = pc
    .start()
    .await;

    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &[0, 1, 0],
            vec![
                AccountMeta::new(my_keypair.pubkey(), false)
            ],
        )],
        Some(&my_keypair.pubkey()),
    );
    transaction.sign(&[&my_keypair], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
}

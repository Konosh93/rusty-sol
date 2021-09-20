use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        _instruction_data: &[u8],
    ) -> ProgramResult {
        msg!("### My Program ID   : {}", program_id);
        let account_info_iter = &mut accounts.iter();
        let singer_acc = next_account_info(account_info_iter)?;
        msg!("### Signer Pub Key: {}", singer_acc.key);

        Ok(())
    }
}

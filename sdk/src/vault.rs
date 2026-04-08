use solana_sdk::pubkey::Pubkey;

pub fn derive_vault_pda(mint: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"vault", mint.as_ref()], program_id)
}

pub fn derive_wick_record_pda(
    vault: &Pubkey,
    index: u64,
    program_id: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"wick", vault.as_ref(), &index.to_le_bytes()],
        program_id,
    )
}

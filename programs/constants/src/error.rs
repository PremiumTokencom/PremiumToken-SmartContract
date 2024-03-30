use anchor_lang::prelude::error_code;

#[error_code]
pub enum TokenCampaignError {
    #[msg("Winner already exists.")]
    WinnerAlreadySelected,
    #[msg("Cannot select a winner because the campaign has no participants.")]
    NoEntries,
    #[msg("Mint address is not same.")]
    InvalidMint,
    #[msg("No enough token holdings.")]
    InsufficientTokens,
}

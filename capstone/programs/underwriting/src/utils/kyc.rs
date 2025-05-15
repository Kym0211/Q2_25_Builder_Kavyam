use anchor_lang::prelude::*;
use crate::error::UnderwritingError;

pub fn verify_kyc(_provider: &AccountInfo, proof: Vec<u8>, _borrower: &Pubkey) -> Result<()> {
    // in production this would call the KYC provider's program
    // For demo purposes, we just check the proof isnt empty

    require!(!proof.is_empty(), UnderwritingError::InvalidKycProvider);

    // ToDo - add more kyc verification 

    Ok(())
}
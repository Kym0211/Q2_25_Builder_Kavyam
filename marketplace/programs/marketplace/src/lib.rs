#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

mod instructions;
mod state;
use instructions::*;
use state::*;

declare_id!("88QZjVK3d64DLRf7sWp4gzY13dzMcufTVMTRSbDACyHo");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16, ) -> Result<()> {
        
        ctx.accounts.init(name, fee, &ctx.bumps)?;
        
        Ok(())
    }

    pub fn list(ctx: Context<List>, price: u64) -> Result<()>{
     
        ctx.accounts.create_listing(price, &ctx.bumps)?;
        ctx.accounts.deposit_nft()?;

        Ok(())
    }

    // pub fn purchase(ctx: Context<Purchase>) -> Result<()> {

    //     Ok(())
    // }
}

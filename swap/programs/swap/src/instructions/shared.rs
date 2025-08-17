use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;


pub fn transfer_tokens<'info> (
    from: &InterfaceAccount<'info, TokenAccount>
) -> Result<()> {

}
// // Contracts/Solana/meme_coin_factory/programs/meme_coin_factory/src/factory.rs
// use anchor_lang::prelude::*;
// use anchor_spl::{
//     associated_token::AssociatedToken,
//     metadata::{
//         create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3,
//         Metadata as Metaplex,
//     },
//     token::{mint_to, Mint, MintTo, Token, TokenAccount},
// };
// use crate::{InitTokenParams, meme_coin_factory}; // Import the necessary items

// declare_id!("GvpMGor7n2G2qqpMJHgkFaqWLNz55STM5ABVCR5D2UJr");

// #[program]
// mod token_factory {
//     use super::*;

//     pub fn create_token(ctx: Context<CreateToken>, name: String, symbol: String, uri: String, decimals: u8) -> Result<()> {
//         // Create a new context for the initiate_token function
//         let token_ctx = Context::<InitToken>::try_from(&ctx)?;

//         // Call the existing token creation logic here
//         let token_address = meme_coin_factory::initiate_token(token_ctx, InitTokenParams { name, symbol, uri, decimals })?;

//         // Add the token address to the user's mapping
//         ctx.accounts.user_tokens.tokens.push(token_address);

//         msg!("Token created and added to user's token list");
//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct CreateToken<'info> {
//     #[account(mut)]
//     pub user: Signer<'info>,
    
//     #[account(
//         init,
//         payer = user,
//         space = 8 + 32 * 10 // Adjust space as needed
//     )]
//     pub user_tokens: Account<'info, UserTokens>,
    
//     pub token_program: Program<'info, Token>,
//     pub system_program: Program<'info, System>,
// }

// #[account]
// #[derive(Default)]
// pub struct UserTokens {
//     pub tokens: Vec<Pubkey>, // Store the addresses of the token contracts
// }
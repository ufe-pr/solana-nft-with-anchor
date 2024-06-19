use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{create_master_edition_v3, create_metadata_accounts_v3, Metadata},
    token::{Mint, Token, TokenAccount},
};

use anchor_spl::metadata::mpl_token_metadata::types::DataV2;
use anchor_spl::metadata::mpl_token_metadata::accounts::{Metadata as MplMetadata, MasterEdition};


declare_id!("FQ47rExrXXeeAJmvcXpEccpq2CutGWCXspuGAzFCjP6L");

#[program]
pub mod solana_nft_with_anchor {
    use anchor_spl::{
        metadata::{
            CreateMasterEditionV3, CreateMetadataAccountsV3,
        },
        token::{mint_to, MintTo},
    };

    use super::*;

    pub fn init_nft(
        ctx: Context<InitNFT>,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        // create mint account
        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.associated_token_account.to_account_info(),
                authority: ctx.accounts.signer.to_account_info(),
            },
        );

        mint_to(cpi_context, 1)?;

        // create metadata account
        let cpi_context = CpiContext::new(
            ctx.accounts.metadata_program.to_account_info(),
            CreateMetadataAccountsV3{
                metadata: ctx.accounts.metadata_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                mint_authority: ctx.accounts.signer.to_account_info(),
                payer: ctx.accounts.signer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
                update_authority: ctx.accounts.signer.to_account_info(),
            },
        );

        let data = DataV2 {
            name,
            symbol,
            uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };

        create_metadata_accounts_v3(cpi_context, data, false, true, None)?;

        create_master_edition_v3(
            CpiContext::new(
                ctx.accounts.metadata_program.to_account_info(),
                CreateMasterEditionV3 {
                    edition: ctx.accounts.master_edition_account.to_account_info(),
                    metadata: ctx.accounts.metadata_account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    mint_authority: ctx.accounts.signer.to_account_info(),
                    update_authority: ctx.accounts.signer.to_account_info(),
                    payer: ctx.accounts.signer.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                },
            ),
            None,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitNFT<'info> {
    /// CHECK: ok, we are passing in this account ourselves
    #[account(mut, signer)]
    pub signer: AccountInfo<'info>,
    #[account(
        init, payer=signer, mint::decimals = 0 , mint::authority = signer.key(), mint::freeze_authority = signer.key(),)]
    pub mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
    )]
    pub associated_token_account: Account<'info, TokenAccount>,

    /// CHECK: is an address
    #[account(
        mut,
        address = MplMetadata::find_pda(&mint.key()).0,
    )]
    pub metadata_account: AccountInfo<'info>,
    /// CHECK: is an address
    #[account(
        mut,
        address = MasterEdition::find_pda(&mint.key()).0,
    )]
    pub master_edition_account: AccountInfo<'info>,

    // Programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub metadata_program: Program<'info, Metadata>,
}

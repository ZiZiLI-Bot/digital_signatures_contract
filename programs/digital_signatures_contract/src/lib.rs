use anchor_lang::prelude::*;
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
use anchor_spl::token::{self, Token, TokenAccount, Transfer as SplTransfer};

declare_id!("HihBXbXfk1N6gucytEDMcCEVzvs3NgePrgsTRCq7Sdzz");

#[program]
pub mod digital_signatures_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, init_data: Storage) -> Result<()> {
        let storage = &mut ctx.accounts.init_storage;

        storage.id = init_data.id;
        storage.name_storage = init_data.name_storage;
        storage.initialized = true;
        storage.counter = 0;
        storage.bump = ctx.bumps.init_storage;

        Ok(())
    }

    pub fn create_signature(ctx: Context<CreateSignature>, sign_data: Signature) -> Result<()> {
        let amount = 10000 * LAMPORTS_PER_SOL;
        let storage = &mut ctx.accounts.storage;
        let signature = &mut ctx.accounts.signature;
        let authority = &ctx.accounts.authority;
        let source = &ctx.accounts.from_ata;
        let destination = &mut ctx.accounts.to_ata;
        let token_program = &ctx.accounts.token_program;

        signature.id = sign_data.id;
        signature.name = sign_data.name;
        signature.url = sign_data.url;
        signature.hash_verified = sign_data.hash_verified;
        signature.state = "unsigned".to_string();

        signature.signature_account = *ctx.accounts.signer_account.key;
        signature.creator_account = *ctx.accounts.signer_account.key;

        signature.bump = ctx.bumps.signature;

        storage.counter += 1;

        let cpi_accounts = SplTransfer {
            from: source.to_account_info().clone(),
            to: destination.to_account_info().clone(),
            authority: authority.to_account_info().clone(),
        };
        let cpi_program = token_program.to_account_info();

        token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;

        Ok(())
    }

    pub fn sign_legal_agreement(ctx: Context<SignLegalAgreement>) -> Result<()> {
        let amount = 10000 * LAMPORTS_PER_SOL;
        let signature = &mut ctx.accounts.signature;
        let authority = &ctx.accounts.authority;
        let source = &ctx.accounts.from_ata;
        let destination = &mut ctx.accounts.to_ata;
        let token_program = &ctx.accounts.token_program;

        signature.state = "signed".to_string();

        let cpi_accounts = SplTransfer {
            from: source.to_account_info().clone(),
            to: destination.to_account_info().clone(),
            authority: authority.to_account_info().clone(),
        };
        let cpi_program = token_program.to_account_info();

        token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;

        Ok(())
    }
}

#[account]
pub struct Storage {
    pub id: String,
    pub name_storage: String,
    pub initialized: bool,
    pub counter: u32,
    pub bump: u8,
}

#[account]
pub struct Signature {
    pub id: String,
    pub name: String,
    pub url: String,
    pub hash_verified: String,
    pub state: String,
    pub signature_account: Pubkey,
    pub creator_account: Pubkey,
    pub bump: u8,
}

// init storage account for the digital legal agreement
// this is storage PDA and count all the legal agreement that are created
// for an individual or business
#[derive(Accounts)]
#[instruction(storage: Storage)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, payer = authority, space = Storage::LEN, seeds = [b"storage", storage.id.as_bytes()], bump)]
    pub init_storage: Account<'info, Storage>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(sign: Signature)]
pub struct CreateSignature<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub storage: Account<'info, Storage>,
    #[account(init, payer = authority, space = Signature::LEN, seeds = [b"signature", storage.key().as_ref(), sign.id.as_ref()], bump)]
    pub signature: Account<'info, Signature>,

    #[account()]
    /// CHECK: the account that is signing the digital agreement
    pub signer_account: AccountInfo<'info>,

    #[account(mut)]
    from_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    to_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct SignLegalAgreement<'info> {
    #[account(mut)]
    /// CHECK: the account that is signing the digital agreement
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub signature: Account<'info, Signature>,

    #[account(mut)]
    from_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    to_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl Storage {
    const LEN: usize = 8 + 1 + 1 + 32 + 80 + 32 + 300;
}

impl Signature {
    const LEN: usize = 32 + 32 + 32 + 32 + 32 + 300 + 8;
}

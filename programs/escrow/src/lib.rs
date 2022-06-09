use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod escrow {
    use anchor_lang::solana_program::{program::invoke, system_instruction};

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        ctx.accounts.my_account.data = data;
        ctx.accounts.my_account.authority = ctx.accounts.authority.key();
        ctx.accounts.my_account.bump = *ctx.bumps.get("my_account").unwrap();
        Ok(())
    }

    pub fn update_data(ctx: Context<UpdateData>, data: u64) -> Result<()> {
        ctx.accounts.my_account.data = data;

        invoke(
            &system_instruction::transfer(
                &ctx.accounts.authority.to_account_info().key(),
                &ctx.accounts.my_account.to_account_info().key(),
                1_000,
            ),
            &[
                ctx.accounts.authority.to_account_info().clone(),
                ctx.accounts.my_account.to_account_info().clone(),
                ctx.accounts.system_program.to_account_info().clone()
            ]
        )?;

        Ok(())
    }

    pub fn withdraw_fee(ctx: Context<WithdrawFee>) -> Result<()> {
        let from_account = &ctx.accounts.my_account.to_account_info();
        let to_account = &ctx.accounts.authority.to_account_info();
        transfer_service_fee_lamports(from_account, to_account, 1_000)?;
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = authority, 
        space = 8 + 8 + 32 + 1,
        seeds = [
            b"my_account".as_ref()
        ],
        bump
    )]
    my_account: Account<'info, MyAccount>,
    #[account(mut)]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut, has_one = authority, seeds = [b"my_account".as_ref()], bump)]
    my_account: Account<'info, MyAccount>,
    #[account(mut)]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawFee<'info> {
    #[account(mut, has_one = authority, seeds = [b"my_account".as_ref()], bump)]
    my_account: Account<'info, MyAccount>,
    #[account(mut)]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

#[account]
pub struct MyAccount {
    data: u64,
    authority: Pubkey,
    bump: u8,
}

#[error_code]
pub enum ErrorCode {
    #[msg("insufficient funds for transaction.")]
    InsufficientFundsForTransaction,
}

fn transfer_service_fee_lamports(
    from_account: &AccountInfo,
    to_account: &AccountInfo,
    amount_of_lamports: u64,
) -> Result<()> {
    // Does the from account have enough lamports to transfer?
    if **from_account.try_borrow_lamports()? < amount_of_lamports {
        return Err(ErrorCode::InsufficientFundsForTransaction.into());
    }
    // Debit from_account and credit to_account
    **from_account.try_borrow_mut_lamports()? -= amount_of_lamports;
    **to_account.try_borrow_mut_lamports()? += amount_of_lamports;
    Ok(())
}
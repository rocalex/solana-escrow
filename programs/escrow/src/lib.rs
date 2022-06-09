use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod escrow {
    use anchor_lang::solana_program::{program::{invoke, invoke_signed}, system_instruction};

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        ctx.accounts.my_account.data = data;
        ctx.accounts.my_account.authority = ctx.accounts.authority.key();
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
        invoke_signed(
            &system_instruction::transfer(
                &ctx.accounts.my_account.to_account_info().key(),
                &ctx.accounts.to.to_account_info().key(),
                1_000,
            ),
            &[
                ctx.accounts.my_account.to_account_info().clone(),
                ctx.accounts.to.to_account_info().clone(),
                ctx.accounts.system_program.to_account_info().clone()
            ],
            &[&[b"my_account".as_ref()]],
        )?;
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = authority, 
        space = 8 + 8 + 32,
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
    /// CHECK:
    #[account(mut)]
    to: AccountInfo<'info>,
    system_program: Program<'info, System>,
}

#[account]
pub struct MyAccount {
    data: u64,
    authority: Pubkey,
}
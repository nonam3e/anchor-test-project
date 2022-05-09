use anchor_lang::prelude::*;

declare_id!("9ex26jHoJ3mvNzHhRTBXGyjbkXQXQX28HfZW8hPC6LM7");

#[program]
pub mod anchor_test_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> anchor_lang::Result<()> {
        let data_account = &mut ctx.accounts.data_account;
        let crowd_account = &mut ctx.accounts.crowd_account;
        let admin_account = & ctx.accounts.admin_account;
        crowd_account.owner_id = admin_account.unsigned_key().clone();
        data_account.owner_id = crowd_account.key().clone();
        data_account.data_list.push(String::from("Created"));
        Ok(())
    }

    pub fn donate(ctx: Context<Donate>, fn_lamports: u64) -> anchor_lang::Result<()> {
        let data_account = &mut ctx.accounts.data_account;
        let patron_account = &mut ctx.accounts.patron_account;
        let crowd_account = &mut ctx.accounts.crowd_account;
        let system_program = ctx.accounts.system_program.to_account_info();
        let patron_lamports = patron_account.lamports();
        let to_trans = anchor_lang::system_program::Transfer{from: patron_account.to_account_info(), to: crowd_account.to_account_info()};
        let result = anchor_lang::system_program::transfer(anchor_lang::context::CpiContext::new(system_program, to_trans), fn_lamports);
        let crowd_new_lamports = patron_lamports - patron_account.to_account_info().lamports();

        let output = format!("Account {} sent {} lamports", &patron_account.unsigned_key(), &crowd_new_lamports);
        if result.is_ok() {data_account.data_list.push(output);}
        result
    }

    pub fn withdraw(ctx: Context<Withdraw>, lamports: u64) -> Result<()> {
        let data_account = &mut ctx.accounts.data_account;
        let crowd_account = &mut ctx.accounts.crowd_account;
        let admin_account = &mut ctx.accounts.admin_account;
        let system_program = ctx.accounts.system_program.to_account_info();
        let admin_lamports = admin_account.lamports();

        if crowd_account.owner_id != *admin_account.unsigned_key() {
            return Err(error!(ErrorCode::Unauthorized));
        }
        let to_trans = anchor_lang::system_program::Transfer{from: crowd_account.to_account_info(), to: admin_account.to_account_info()};
        let result = anchor_lang::system_program::transfer(anchor_lang::context::CpiContext::new(system_program, to_trans), lamports);
        let admin_new_lamports = admin_account.lamports() - admin_lamports;

        let output = format!("{} lamports withdrew to admin", &admin_new_lamports);
        if result.is_ok() {data_account.data_list.push(output);}
        result
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = admin_account, space = 128 + 128)] //can I use zero instead?
    pub data_account: Account<'info, DataAccount>,
    #[account(init, payer = admin_account, space = 8 + 32)]
    pub crowd_account: Account<'info, CrowdAccount>,
    #[account(mut)]
    pub admin_account: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct Donate<'info> {
    #[account(mut)]
    pub data_account: Account<'info, DataAccount>,
    #[account(mut)]
    pub patron_account: Signer<'info>,
    #[account(mut)]
    pub crowd_account: Account<'info, CrowdAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub crowd_account: Account<'info, CrowdAccount>,
    #[account(mut)]
    pub admin_account: Signer<'info>,
    #[account(mut)]
    pub data_account: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data_list: Vec<String>,
    pub owner_id: Pubkey,
}
#[account]
pub struct CrowdAccount {
    pub owner_id: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("You are not permitted to perform this action.")]
    Unauthorized,
}

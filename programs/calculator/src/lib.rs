use anchor_lang::prelude::*;

// @dev declare_id - a macro used for declaring the program’s on-chain address
declare_id!("48SJYGxx9ryNivustKbfdTCULDzo7JMfbxQAu9Z26Q2d");

// @dev #[program] - attribute macro used to denote the module containing the program’s instruction logic
#[program]
pub mod calculator {
    use super::*;
    pub fn create(ctx: Context<Create>, init_message: String) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;
        Ok(())
    }
    pub fn add(ctx: Context<Addition>, num1: i64, num2: i64) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 + num2;
        Ok(())
    }

    pub fn subtract(ctx: Context<Subtraction>, num1: i64, num2: i64) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 - num2;
        Ok(())
    }

    pub fn multiply(ctx: Context<Multiplication>, num1: i64, num2: i64) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 * num2;
        Ok(())
    }

    pub fn divide(ctx: Context<Division>, num1: i64, num2: i64) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;

        calculator.result = num1 / num2;
        calculator.remainder = num1 % num2;
        Ok(())
    }
}

// @dev Context in above code is defined by the Create struct below
#[derive(Accounts)]
pub struct Create<'info> {
    // @dev init - denotes the initialization of new "calculator" account
    // @dev payer - denotes the account that is doing the transaction, which is also the account that will pay the gas fees
    // @dev space - denotes the amount of space to be allocated on solana blockchain for this account
    #[account(init, payer = user, space = 264)]
    pub calculator: Account<'info, Calculator>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Addition<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Subtraction<'info> {
    #[account(mut)] // defines the account as mutable
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Multiplication<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Division<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[account]
pub struct Calculator {
    pub greeting: String,
    pub result: i64,
    pub remainder: i64,
}

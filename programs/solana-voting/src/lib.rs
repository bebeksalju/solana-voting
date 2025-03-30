use anchor_lang::prelude::*;

declare_id!("LGoasg9Ubv5oHQP4LBntgzzckY5sJp4Mh9nU9jargDW");

#[program]
pub mod solana_voting {
    use super::*;

    pub fn create_poll(ctx: Context<CreatePoll>, title: String) -> Result<()> {
        let poll = &mut ctx.accounts.poll;
        poll.title = title;
        poll.yes_votes = 0;
        poll.no_votes = 0;
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, vote_yes: bool) -> Result<()> {
        let poll = &mut ctx.accounts.poll;
        if vote_yes {
            poll.yes_votes += 1;
        } else {
            poll.no_votes += 1;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreatePoll<'info> {
    #[account(init, payer = user, space = 128)]
    pub poll: Account<'info, Poll>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub poll: Account<'info, Poll>,
}

#[account]
pub struct Poll {
    pub title: String,
    pub yes_votes: u64,
    pub no_votes: u64,
}


#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, Address, symbol_short};

#[contracttype]
#[derive(Clone)]
pub struct RewardRecord {
    pub course_id: String,
    pub tokens_earned: u64,
}

#[contracttype]
pub enum RewardBook {
    User(Address, String), // (User, CourseID)
}

#[contract]
pub struct CourseTokenRewardSystem;

#[contractimpl]
impl CourseTokenRewardSystem {
    pub fn reward_user(env: Env, user: Address, course_id: String, tokens: u64) {
        let key = RewardBook::User(user.clone(), course_id.clone());

        let mut record = env.storage().instance().get(&key).unwrap_or(RewardRecord {
            course_id: course_id.clone(),
            tokens_earned: 0,
        });

        record.tokens_earned += tokens;

        env.storage().instance().set(&key, &record);
    }

    pub fn view_rewards(env: Env, user: Address, course_id: String) -> RewardRecord {
        let key = RewardBook::User(user, course_id.clone());

        env.storage().instance().get(&key).unwrap_or(RewardRecord {
            course_id,
            tokens_earned: 0,
        })
    }
}

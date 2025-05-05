#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol, Vec, log};

const FAUCET_AMOUNT: u64 = 100;
const CLAIMED_USERS: Symbol = symbol_short!("CLAIMED");

#[contract]
pub struct TokenFaucet;

#[contractimpl]
impl TokenFaucet {
    // Allow a user to claim faucet tokens once
    pub fn claim(env: Env, user: Address) -> u64 {
        user.require_auth();

        let mut claimed: Vec<Address> = env
            .storage()
            .instance()
            .get(&CLAIMED_USERS)
            .unwrap_or(Vec::new(&env));

        if claimed.contains(&user) {
            log!(&env, "User already claimed tokens");
            panic!("Already claimed");
        }

        claimed.push_back(user.clone());
        env.storage().instance().set(&CLAIMED_USERS, &claimed);

        log!(&env, "User {:?} claimed {} tokens", user, FAUCET_AMOUNT);

        FAUCET_AMOUNT
    }

    // Check if user has already claimed tokens
    pub fn has_claimed(env: Env, user: Address) -> bool {
        let claimed: Vec<Address> = env
            .storage()
            .instance()
            .get(&CLAIMED_USERS)
            .unwrap_or(Vec::new(&env));

        claimed.contains(&user)
    }

    // Get list of all users who have claimed
    pub fn get_claimed_users(env: Env) -> Vec<Address> {
        env.storage()
            .instance()
            .get(&CLAIMED_USERS)
            .unwrap_or(Vec::new(&env))
    }
}

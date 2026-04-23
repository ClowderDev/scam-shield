#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, Symbol, Vec, Map,
};

#[contracttype]
#[derive(Clone)]
pub struct Policy {
    pub owner: Address,
    pub coverage: i128,
    pub premium: i128,
    pub active: bool,
}

#[contracttype]
#[derive(Clone)]
pub struct Claim {
    pub claimant: Address,
    pub amount: i128,
    pub approved: bool,
    pub votes_for: u32,
    pub votes_against: u32,
}

#[contract]
pub struct InsuranceContract;

#[contractimpl]
impl InsuranceContract {

    // Buy insurance policy
    pub fn buy_policy(env: Env, user: Address, coverage: i128, premium: i128) {
        user.require_auth();

        let policy = Policy {
            owner: user.clone(),
            coverage,
            premium,
            active: true,
        };

        env.storage().instance().set(&user, &policy);
    }

    // Submit claim
    pub fn submit_claim(env: Env, user: Address, amount: i128) {
        user.require_auth();

        let claim = Claim {
            claimant: user.clone(),
            amount,
            approved: false,
            votes_for: 0,
            votes_against: 0,
        };

        let key = (Symbol::short("CLAIM"), user.clone());
        env.storage().instance().set(&key, &claim);
    }

    // Vote on claim (DAO-style)
    pub fn vote_claim(env: Env, voter: Address, claimant: Address, approve: bool) {
        voter.require_auth();

        let key = (Symbol::short("CLAIM"), claimant.clone());
        let mut claim: Claim = env.storage().instance().get(&key).unwrap();

        if approve {
            claim.votes_for += 1;
        } else {
            claim.votes_against += 1;
        }

        env.storage().instance().set(&key, &claim);
    }

    // Resolve claim
    pub fn resolve_claim(env: Env, claimant: Address) {
        let key = (Symbol::short("CLAIM"), claimant.clone());
        let mut claim: Claim = env.storage().instance().get(&key).unwrap();

        if claim.votes_for > claim.votes_against {
            claim.approved = true;
        }

        env.storage().instance().set(&key, &claim);
    }

    // Payout
    pub fn payout(env: Env, claimant: Address) {
        claimant.require_auth();

        let key = (Symbol::short("CLAIM"), claimant.clone());
        let claim: Claim = env.storage().instance().get(&key).unwrap();

        if !claim.approved {
            panic!("Claim not approved");
        }

        // NOTE: token transfer logic should be added here
        // using Stellar token interface

    }
}

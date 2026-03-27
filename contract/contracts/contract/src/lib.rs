#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol, Address, Vec, Map};

// Storage keys
const TOTAL_REVENUE: Symbol = symbol_short!("TOTAL");

#[contract]
pub struct RevenueSharing;

#[contractimpl]
impl RevenueSharing {

    // Initialize contract with participants and their share percentages
    pub fn init(env: Env, participants: Vec<Address>, shares: Vec<u32>) {
        assert!(participants.len() == shares.len(), "Mismatch");

        let mut map = Map::new(&env);

        for i in 0..participants.len() {
            map.set(participants.get(i).unwrap(), shares.get(i).unwrap());
        }

        env.storage().instance().set(&symbol_short!("SHARES"), &map);
        env.storage().instance().set(&TOTAL_REVENUE, &0u64);
    }

    // Add revenue to contract
    pub fn deposit(env: Env, amount: u64) {
        let mut total: u64 = env
            .storage()
            .instance()
            .get(&TOTAL_REVENUE)
            .unwrap_or(0);

        total += amount;

        env.storage().instance().set(&TOTAL_REVENUE, &total);
    }

    // Distribute revenue based on shares
    pub fn distribute(env: Env) -> Map<Address, u64> {
        let total: u64 = env
            .storage()
            .instance()
            .get(&TOTAL_REVENUE)
            .unwrap_or(0);

        let shares: Map<Address, u32> = env
            .storage()
            .instance()
            .get(&symbol_short!("SHARES"))
            .unwrap();

        let mut payouts = Map::new(&env);

        let mut total_share: u32 = 0;

        for (_, share) in shares.iter() {
            total_share += share;
        }

        for (addr, share) in shares.iter() {
            let payout = (total * share as u64) / total_share as u64;
            payouts.set(addr, payout);
        }

        payouts
    }
}
#![no_std]

use soroban_sdk::{contract, contractimpl, Env, String, Vec};

#[contract]
pub struct TarotContract;

#[contractimpl]
impl TarotContract {
    pub fn init_user(env: Env, user: String) {
        env.storage().persistent().set(&user, &0i32);
    }

    pub fn topup(env: Env, user: String, amount: i32) {
        let current: i32 = env.storage().persistent().get(&user).unwrap_or(0);
        env.storage().persistent().set(&user, &(current + amount));
    }

    pub fn draw_cards(env: Env, user: String, num_cards: u32) -> Vec<String> {
        let current: i32 = env.storage().persistent().get(&user).unwrap_or(0);
        
        if current <= 0 {
            return Vec::new(&env);
        }
        
        env.storage().persistent().set(&user, &(current - 1));
        
        let mut result = Vec::new(&env);
        
        for i in 0..num_cards {
            let card = match i % 8 {
                0 => String::from_str(&env, "The Fool"),
                1 => String::from_str(&env, "The Magician"),
                2 => String::from_str(&env, "The High Priestess"),
                3 => String::from_str(&env, "The Lovers"),
                4 => String::from_str(&env, "Death"),
                5 => String::from_str(&env, "The Sun"),
                6 => String::from_str(&env, "The Moon"),
                _ => String::from_str(&env, "The World"),
            };
            result.push_back(card);
        }
        
        result
    }
}
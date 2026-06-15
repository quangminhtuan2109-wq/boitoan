# TarotContract

A simple smart contract built with Soroban SDK in Rust that manages users, credits, and Tarot card draws.

## Features
- Initialize User: Register a new user with zero credits.
- Top-up Credits: Add credits to a user’s account.
- Draw Tarot Cards: Spend credits to draw cards from a fixed Tarot set.

## Contract Functions

### `init_user`
```rust
pub fn init_user(env: Env, user: String)

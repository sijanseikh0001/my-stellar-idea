#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address, String, Map, symbol_short};

#[contract]
pub struct LocationRewards;

#[contractimpl]
impl LocationRewards {

    // User check-in at a location
    pub fn check_in(env: Env, user: Address, location: String) {
        // Require user authentication
        user.require_auth();

        // Key for storing visited locations
        let key = (user.clone(), symbol_short!("LOC"));

        // Get stored locations or create new map
        let mut locations: Map<String, bool> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(Map::new(&env));

        // Prevent duplicate check-in
        if locations.contains_key(location.clone()) {
            panic!("Already checked in at this location");
        }

        // Save location
        locations.set(location, true);
        env.storage().persistent().set(&key, &locations);

        // Add reward points
        let pts_key = (user.clone(), symbol_short!("PTS"));
        let mut points: i32 = env
            .storage()
            .persistent()
            .get(&pts_key)
            .unwrap_or(0);

        points += 10; // reward per check-in
        env.storage().persistent().set(&pts_key, &points);
    }

    // Get user points
    pub fn get_points(env: Env, user: Address) -> i32 {
        let pts_key = (user, symbol_short!("PTS"));
        env.storage().persistent().get(&pts_key).unwrap_or(0)
    }
}
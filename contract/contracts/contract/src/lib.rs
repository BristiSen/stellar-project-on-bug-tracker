#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Symbol, Vec};

#[contracttype]
#[derive(Clone)]
pub struct Bug {
    pub reporter: Address,
    pub description: String,
    pub status: Symbol,
}

#[contracttype]
pub enum DataKey {
    Bug(Symbol),
    BugList,
}

#[contract]
pub struct BugTracker;

#[contractimpl]
impl BugTracker {

    pub fn create_bug(env: Env, id: Symbol, reporter: Address, description: String) {
        reporter.require_auth();

        let bug = Bug {
            reporter,
            description,
            status: Symbol::new(&env, "open"),
        };

        env.storage().instance().set(&DataKey::Bug(id.clone()), &bug);

        let mut bugs: Vec<Symbol> = env
            .storage()
            .instance()
            .get(&DataKey::BugList)
            .unwrap_or(Vec::new(&env));

        bugs.push_back(id);
        env.storage().instance().set(&DataKey::BugList, &bugs);
    }

    pub fn close_bug(env: Env, id: Symbol) {
        let mut bug: Bug = env
            .storage()
            .instance()
            .get(&DataKey::Bug(id.clone()))
            .unwrap();

        bug.status = Symbol::new(&env, "closed");

        env.storage().instance().set(&DataKey::Bug(id), &bug);
    }

    pub fn get_bug(env: Env, id: Symbol) -> Bug {
        env.storage().instance().get(&DataKey::Bug(id)).unwrap()
    }

    pub fn list_bugs(env: Env) -> Vec<Symbol> {
        env.storage().instance().get(&DataKey::BugList).unwrap_or(Vec::new(&env))
    }
}
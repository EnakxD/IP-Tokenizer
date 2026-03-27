#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol, Vec,
};

#[contracttype]
#[derive(Clone)]
pub struct IPAsset {
    pub creator: Address,
    pub metadata: Symbol, // simple identifier (could be IPFS hash)
    pub owner: Address,
}

#[contract]
pub struct IPTokenizationContract;

#[contractimpl]
impl IPTokenizationContract {

    // Register a new IP asset
    pub fn register_ip(env: Env, creator: Address, metadata: Symbol) {
        creator.require_auth();

        let asset = IPAsset {
            creator: creator.clone(),
            metadata: metadata.clone(),
            owner: creator.clone(),
        };

        env.storage().instance().set(&metadata, &asset);
    }

    // Get IP details
    pub fn get_ip(env: Env, metadata: Symbol) -> IPAsset {
        env.storage()
            .instance()
            .get(&metadata)
            .unwrap()
    }

    // Transfer ownership
    pub fn transfer_ip(env: Env, metadata: Symbol, new_owner: Address) {
        let mut asset: IPAsset = env
            .storage()
            .instance()
            .get(&metadata)
            .unwrap();

        asset.owner.require_auth();

        asset.owner = new_owner.clone();

        env.storage().instance().set(&metadata, &asset);
    }
}
#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, vec, Address, Env, Symbol, Vec};

#[contract]
pub struct RentalContract;

#[contractimpl]
impl RentalContract {
    pub fn create_agreement(env: Env, tenant: Address, landlord: Address, deposit: i64, is_active: bool) -> Vec<Symbol> {
        let agreement_key = (tenant.clone(), landlord.clone());

        env.storage().instance().set(&(agreement_key.clone(), "tenant"), &tenant);
        env.storage().instance().set(&(agreement_key.clone(), "landlord"), &landlord);
        env.storage().instance().set(&(agreement_key.clone(), "deposit"), &deposit);
        env.storage().instance().set(&(agreement_key, "is_active"), &is_active);

        vec![&env, symbol_short!("Success")]
    }

    pub fn get_agreement(env: Env, tenant: Address, landlord: Address) -> Option<(Address, Address, i64, bool)> {
        let agreement_key = (tenant.clone(), landlord.clone());

        let tenant: Address = env.storage().instance().get(&(agreement_key.clone(), "tenant"))?;
        let landlord: Address = env.storage().instance().get(&(agreement_key.clone(), "landlord"))?;
        let deposit: i64 = env.storage().instance().get(&(agreement_key.clone(), "deposit"))?;
        let is_active: bool = env.storage().instance().get(&(agreement_key, "is_active"))?;

        Some((tenant, landlord, deposit, is_active))
    }
}

mod test;

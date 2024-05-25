#![cfg(test)]

use super::*;
use soroban_sdk::{
    symbol_short,
    testutils::Address as _,
    Address, Env
};

#[test]
fn test_create_and_get_agreement() {
    let env = Env::default();

    let tenant = Address::generate(&env);
    let landlord = Address::generate(&env);
    let deposit: i64 = 1000;
    let is_active: bool = true;

    let result = RentalContract::create_agreement(env.clone(), tenant.clone(), landlord.clone(), deposit, is_active);
    assert_eq!(result, vec![&env, symbol_short!("Success")]);

    let agreement = RentalContract::get_agreement(env.clone(), tenant.clone(), landlord.clone()).unwrap();

    assert_eq!(agreement.0, tenant);
    assert_eq!(agreement.1, landlord);
    assert_eq!(agreement.2, deposit);
    assert_eq!(agreement.3, is_active);
}

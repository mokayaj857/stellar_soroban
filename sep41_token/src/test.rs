#![cfg(test)]
extern crate std;

use crate::{contract::Token, TokenClient};
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    Address, Env, FromVal, IntoVal, String, Symbol,
};

fn create_token<'a>(
    e: &Env,
    admin: &Address,
    initial_holder: &Address,
    initial_mint: i128,
) -> TokenClient<'a> {
    let token_contract = e.register(
        Token,
        (
            admin,
            initial_holder,
            initial_mint,
            7_u32,
            String::from_val(e, &"SEP41 Token"),
            String::from_val(e, &"SEP41"),
        ),
    );
    TokenClient::new(e, &token_contract)
}

#[test]
fn test_constructor_mint() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let holder = Address::generate(&e);
    let token = create_token(&e, &admin, &holder, 10_000);

    assert_eq!(token.balance(&holder), 10_000);
    assert_eq!(token.decimals(), 7);
    assert_eq!(token.name(), String::from_val(&e, &"SEP41 Token"));
    assert_eq!(token.symbol(), String::from_val(&e, &"SEP41"));
}

#[test]
fn test_mint() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let holder = Address::generate(&e);
    let recipient = Address::generate(&e);
    let token = create_token(&e, &admin, &holder, 0);

    token.mint(&recipient, &500);
    assert_eq!(
        e.auths(),
        std::vec![(
            admin.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("mint"),
                    (&recipient, 500_i128).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(token.balance(&recipient), 500);
}

#[test]
fn test_transfer_from() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let owner = Address::generate(&e);
    let spender = Address::generate(&e);
    let recipient = Address::generate(&e);
    let token = create_token(&e, &admin, &owner, 1000);

    token.approve(&owner, &spender, &400, &200);
    assert_eq!(token.allowance(&owner, &spender), 400);

    token.transfer_from(&spender, &owner, &recipient, &300);
    assert_eq!(
        e.auths(),
        std::vec![(
            spender.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    Symbol::new(&e, "transfer_from"),
                    (&spender, &owner, &recipient, 300_i128).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    assert_eq!(token.balance(&owner), 700);
    assert_eq!(token.balance(&recipient), 300);
    assert_eq!(token.allowance(&owner, &spender), 100);
}

#[test]
fn test_burn() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let holder = Address::generate(&e);
    let token = create_token(&e, &admin, &holder, 1000);

    token.burn(&holder, &250);
    assert_eq!(
        e.auths(),
        std::vec![(
            holder.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("burn"),
                    (&holder, 250_i128).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(token.balance(&holder), 750);
}

#[test]
fn test_burn_from() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let owner = Address::generate(&e);
    let spender = Address::generate(&e);
    let token = create_token(&e, &admin, &owner, 1000);

    token.approve(&owner, &spender, &600, &200);
    assert_eq!(token.allowance(&owner, &spender), 600);

    token.burn_from(&spender, &owner, &400);
    assert_eq!(
        e.auths(),
        std::vec![(
            spender.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("burn_from"),
                    (&spender, &owner, 400_i128).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    assert_eq!(token.balance(&owner), 600);
    assert_eq!(token.allowance(&owner, &spender), 200);
}

#[test]
#[should_panic(expected = "insufficient allowance")]
fn transfer_from_insufficient_allowance() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let owner = Address::generate(&e);
    let spender = Address::generate(&e);
    let recipient = Address::generate(&e);
    let token = create_token(&e, &admin, &owner, 1000);

    token.approve(&owner, &spender, &50, &200);
    token.transfer_from(&spender, &owner, &recipient, &51);
}

#[test]
#[should_panic(expected = "insufficient balance")]
fn burn_insufficient_balance() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let holder = Address::generate(&e);
    let token = create_token(&e, &admin, &holder, 100);

    token.burn(&holder, &101);
}

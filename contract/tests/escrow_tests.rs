use megron::MegronContractClient;
use soroban_sdk::{
    testutils::{Address as _, Ledger, LedgerInfo},
    token::{Client as TokenClient, StellarAssetClient},
    Address, Env,
};

fn setup() -> (Env, MegronContractClient<'static>, Address, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    env.ledger().set(LedgerInfo {
        timestamp: 1_000_000,
        protocol_version: 25,
        sequence_number: 100,
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_ttl: 1,
        min_persistent_entry_ttl: 1,
        max_entry_ttl: 6_312_000,
    });

    let contract_id = env.register(megron::MegronContract, ());
    let client = MegronContractClient::new(&env, &contract_id);

    let client_addr = Address::generate(&env);
    let worker = Address::generate(&env);

    let token_admin = Address::generate(&env);
    let token_id = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_address = token_id.address();
    StellarAssetClient::new(&env, &token_address).mint(&client_addr, &10_000);

    (env, client, client_addr, worker, token_address)
}

#[test]
fn test_fund_transfers_to_contract() {
    let (env, contract, client_addr, worker, token) = setup();

    contract.fund(&client_addr, &worker, &token, &1_000);

    let token_client = TokenClient::new(&env, &token);
    assert_eq!(token_client.balance(&client_addr), 9_000);

    let escrow = contract.get_escrow();
    assert_eq!(escrow.amount, 1_000);
}

#[test]
fn test_release_pays_worker() {
    let (env, contract, client_addr, worker, token) = setup();

    contract.fund(&client_addr, &worker, &token, &1_000);
    let released = contract.release();

    assert_eq!(released, 1_000);
    assert_eq!(TokenClient::new(&env, &token).balance(&worker), 1_000);
}

#[test]
fn test_refund_returns_to_client() {
    let (env, contract, client_addr, worker, token) = setup();

    contract.fund(&client_addr, &worker, &token, &1_000);
    let refunded = contract.refund();

    assert_eq!(refunded, 1_000);
    assert_eq!(TokenClient::new(&env, &token).balance(&client_addr), 10_000);
}

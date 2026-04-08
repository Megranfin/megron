use megron::MegronContractClient;
use soroban_sdk::{
    testutils::{Address as _, Ledger, LedgerInfo},
    token::{Client as TokenClient, StellarAssetClient},
    Address, Env,
};

fn setup() -> (Env, MegronContractClient<'static>, Address, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    // Set protocol version to 25 (required by soroban-sdk v25) before any calls
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

    let subscriber = Address::generate(&env);
    let recipient = Address::generate(&env);

    let token_admin = Address::generate(&env);
    let token_id = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_address = token_id.address();
    StellarAssetClient::new(&env, &token_address).mint(&subscriber, &10_000);

    (env, client, subscriber, recipient, token_address)
}

#[test]
fn test_subscribe_creates_subscription() {
    let (_env, client, subscriber, recipient, token) = setup();

    let sub = client.subscribe(&subscriber, &recipient, &token, &500, &2_592_000);

    assert_eq!(sub.subscriber, subscriber);
    assert_eq!(sub.amount, 500);
    assert!(sub.active);
}

#[test]
fn test_charge_transfers_and_advances_schedule() {
    let (env, client, subscriber, recipient, token) = setup();

    let interval = 2_592_000_u64;
    client.subscribe(&subscriber, &recipient, &token, &500, &interval);

    // Approve contract to pull from subscriber's allowance
    let token_client = TokenClient::new(&env, &token);
    token_client.approve(&subscriber, &client.address, &10_000, &(env.ledger().sequence() + 10_000));

    // Advance time past the first due date
    let subscribe_time = env.ledger().timestamp();
    env.ledger().set(LedgerInfo {
        timestamp: subscribe_time + interval + 1,
        protocol_version: 25,
        sequence_number: env.ledger().sequence(),
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_ttl: 1,
        min_persistent_entry_ttl: 1,
        max_entry_ttl: 6_312_000,
    });

    let charged = client.charge();
    assert_eq!(charged, 500);
    assert_eq!(token_client.balance(&recipient), 500);

    let sub = client.get_subscription();
    // next_payment was (subscribe_time + interval), after charge it advances by interval
    assert_eq!(sub.next_payment, subscribe_time + interval + interval);
}

#[test]
fn test_cancel_deactivates_subscription() {
    let (_env, client, subscriber, recipient, token) = setup();

    client.subscribe(&subscriber, &recipient, &token, &500, &2_592_000);
    client.cancel();

    let sub = client.get_subscription();
    assert!(!sub.active);
}

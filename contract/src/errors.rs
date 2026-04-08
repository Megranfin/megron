use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum MegronError {
    // Subscription errors
    AlreadySubscribed = 1,
    SubscriptionNotFound = 2,
    SubscriptionInactive = 3,
    PaymentNotDue = 4,
    Unauthorized = 5,
    // Escrow errors
    EscrowNotFound = 6,
    EscrowNotFunded = 7,
    InvalidAmount = 8,
}

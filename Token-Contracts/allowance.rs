use crate::storage_types::{AllowanceDataKey, AllowanceValue, DataKey};
use soroban_sdk::{Address, Env};

/// Reads the allowance from `from` to `spender`.
pub fn read_allowance(e: &Env, from: Address, spender: Address) -> AllowanceValue {
    let key = DataKey::Allowance(AllowanceDataKey {
        from: from.clone(),
        spender: spender.clone(),
    });

    if let Some(allowance) = e.storage().temporary().get::<_, AllowanceValue>(&key) {
        if allowance.expiration_ledger < e.ledger().sequence() {
            AllowanceValue {
                amount: 0,
                expiration_ledger: allowance.expiration_ledger,
            }
        } else {
            allowance
        }
    } else {
        AllowanceValue {
            amount: 0,
            expiration_ledger: 0,
        }
    }
}

/// Writes or updates the allowance value.
pub fn write_allowance(
    e: &Env,
    from: Address,
    spender: Address,
    amount: i128,
    expiration_ledger: u32,
) {
    let key = DataKey::Allowance(AllowanceDataKey {
        from: from.clone(),
        spender: spender.clone(),
    });

    let allowance = AllowanceValue {
        amount,
        expiration_ledger,
    };

    e.storage().temporary().set(&key, &allowance);

    if amount > 0 {
        let live_for = expiration_ledger
            .checked_sub(e.ledger().sequence())
            .unwrap_or(0); // Avoid panic
        e.storage().temporary().extend_ttl(&key, live_for, live_for);
    }
}

/// Deducts allowance if sufficient, else panics.
pub fn spend_allowance(e: &Env, from: Address, spender: Address, amount: i128) {
    let allowance = read_allowance(e, from.clone(), spender.clone());

    if allowance.amount < amount {
        panic!("insufficient allowance");
    }

    // Update storage with reduced allowance
    let remaining = allowance.amount - amount;
    write_allowance(e, from, spender, remaining, allowance.expiration_ledger);
}

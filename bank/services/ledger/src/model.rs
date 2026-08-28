// ledger — domain types (entities, value objects)
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Debug)]
pub struct Money(i64);

impl Money {
    pub fn from_cents(cents: i64) -> Money {
        Money(cents)
    }
    pub fn cents(&self) -> i64 {
        self.0
    }
    pub fn add(&self, other: Money) -> Option<Money> {
        match self.0.checked_add(other.0) {
            Some(sum) => Some(Money(sum)),
            None => None,
        }
    }
}
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum AccountType {
    Checkings,
    Savings,
}
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum AccountStatus {
    Active,
    Frozen,
    Closed,
}
#[derive(Clone, Debug)]
pub struct Account {
    account_id: u64,
    owner_id: u64,
    account_type: AccountType,
    status: AccountStatus,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Entry {
    entry_id: u64,
    account_id: u64,
    amount: Money,
    transaction_id: u64
}
#[derive(Clone, Debug)]
pub struct Transaction {
    transaction_id: u64,
    entries: Vec<Entry>,
    description: String,
}
impl Transaction {
    pub fn is_balanced(&self) -> bool {
        let mut total: i64 = 0;
        for entry in &self.entries {
            total += entry.amount.cents();
        }
        total == 0
    }
}
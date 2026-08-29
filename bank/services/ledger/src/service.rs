// ledger — business logic
use crate::model::{Transaction, Money, Entry, Account};

#[derive(Debug)]
pub enum LedgerError {
    Unbalanced,
    AccountFrozen,
    AccountNotFound,
}
pub fn post(transaction: Transaction) -> Result<(), LedgerError> {
    if !transaction.is_balanced() {
        return Err(LedgerError::Unbalanced);
    }
    // todo, check account arent frozen
    // hand to repository for atomic db write

    Ok(())
}
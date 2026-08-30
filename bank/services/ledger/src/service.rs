// ledger — business logic
use crate::model::{Transaction, Money, Entry, Account};
use crate::repository::LedgerRepository;

#[derive(Debug)]
pub enum LedgerError {
    Unbalanced,
    AccountFrozen,
    AccountNotFound,
}
pub async fn post(transaction: Transaction) -> Result<(), LedgerError> {
    if !transaction.is_balanced() {
        return Err(LedgerError::Unbalanced);
    }
    // todo, check account arent frozen
    // hand to repository for atomic db write

    Ok(())
}
<<<<<<< HEAD
pub struct LedgerService {
    repo: LedgerRepository,
}
impl LedgerSerivce {
    pub fn new(repo: LedgerRepository) -> LedgerService {
        LedgerService ( repo )
    }
}

=======
>>>>>>> 08610bacdd2738865a661c621be03e5193afd327

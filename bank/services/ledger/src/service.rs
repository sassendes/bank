// ledger — business logic
use crate::model::Transaction;
use crate::repository::LedgerRepository;

#[derive(Debug)]
pub enum LedgerError {
    Unbalanced,
    AccountFrozen,
    AccountNotFound,
    Database(sqlx::Error),
}

pub struct LedgerService {
    repo: LedgerRepository,
}

impl LedgerService {
    pub fn new(repo: LedgerRepository) -> LedgerService {
        LedgerService { repo }
    }

    pub async fn post(&self, transaction: Transaction) -> Result<(), LedgerError> {
        if !transaction.is_balanced() {
            return Err(LedgerError::Unbalanced);
        }

        self.repo.post(&transaction)
            .await
            .map_err(LedgerError::Database)?;

        Ok(())
    }
}

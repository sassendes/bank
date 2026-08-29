// ledger — persistence (DB access)
use crate::model::{Transaction, Money};
use sqlx::PgPool;

// most important struct
pub struct LedgerRepository {
    pool: PgPool,
}

impl LedgerRepository {
    pub fn new(pool: PgPool) -> LedgerRepository {
        LedgerRepository { pool }
    }
    pub async fn balance(&self, account_id: i64) -> Result<Money, sqlx::Error> {
    let row: (Option<i64>,) = sqlx::query_as(
        "SELECT SUM(amount) FROM entries WHERE account_id = $1"
    )
    .bind(account_id)
    .fetch_one(&self.pool)
    .await?;

    let cents = row.0.unwrap_or(0);
    Ok(Money::from_cents(cents))
    }
    pub async fn post(&self, transaction: &Transaction) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        for entry in &transaction.entries {
            sqlx::query(
                "INSERT INTO entries (entry_id, account_id, amount, transaction_id) VALUES ($1, $2, $3, $4)"
            )
            .bind(entry.entry_id as i64)
            .bind(entry.account_id as i64)
            .bind(entry.amount.cents())
            .bind(entry.transaction_id as i64)
            .execute(&mut *tx)
            .await?;
        }
        tx.commit().await?;
        Ok(())
    }
}

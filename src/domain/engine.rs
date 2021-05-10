use crate::domain::model::{
    Chargeback, Client, Deposit, Dispute, Resolve, Transaction, TransactionStatus, Withdrawal,
};
use crate::domain::ports::{
    ClientRepository, ClientUpdate, Engine, EngineConfig, EngineErrors, EngineResult,
    TransactionsRepository,
};
use async_trait::async_trait;
use futures::prelude::stream::BoxStream;
use futures::{StreamExt, TryStreamExt};

#[derive(Default, Debug)]
pub struct TransactionEngine<T: EngineConfig> {
    clients: T::ClientRepository,
    transactions: T::TransactionRepository,
}

#[async_trait]
impl<T> Engine for TransactionEngine<T>
where
    T: EngineConfig,
{
    async fn process_transaction(&mut self, transaction: Transaction) -> EngineResult {
        match transaction {
            Transaction::Deposit(deposit) => self.process_deposit(deposit).await,
            Transaction::Withdrawal(withdrawal) => self.process_withdrawal(withdrawal).await,
            Transaction::Dispute(dispute) => self.process_dispute(dispute).await,
            Transaction::Resolve(resolve) => self.process_resolve(resolve).await,
            Transaction::Chargeback(chargeback) => self.process_chargeback(chargeback).await,
        }
    }

    async fn get_clients(
        &self,
    ) -> Result<BoxStream<'static, Result<Client, EngineErrors>>, EngineErrors> {
        Ok(self
            .clients
            .get_all()
            .await?
            .map_err(|e| EngineErrors::ClientError(e))
            .boxed())
    }
}

impl<T> TransactionEngine<T>
where
    T: EngineConfig,
{
    async fn process_deposit(&mut self, deposit: Deposit) -> EngineResult {
        self.transactions
            .store_transaction_value(deposit.tx, deposit.amount.clone())
            .await?;
        self.transactions
            .store_transaction_status(deposit.tx, TransactionStatus::Processed)
            .await?;
        self.clients
            .update(
                &deposit.client,
                ClientUpdate::Deposit {
                    available_increase: deposit.amount.clone(),
                    total_increase: deposit.amount,
                },
            )
            .await?;
        Ok(())
    }

    async fn process_withdrawal(&mut self, withdrawal: Withdrawal) -> EngineResult {
        let client = self.clients.get(&withdrawal.client).await?;
        if client.available > withdrawal.amount {
            self.clients
                .update(
                    &withdrawal.client,
                    ClientUpdate::Withdrawal {
                        available_decrease: withdrawal.amount.clone(),
                        total_decrease: withdrawal.amount.clone(),
                    },
                )
                .await?;
        }
        Ok(())
    }

    async fn process_dispute(&mut self, dispute: Dispute) -> EngineResult {
        let status = self
            .transactions
            .get_transaction_status(&dispute.tx)
            .await?;

        let value = self.transactions.get_transaction_value(&dispute.tx).await?;

        // Only handle dispute if transaction is in the base processed state
        if status == TransactionStatus::Processed {
            self.transactions
                .store_transaction_status(dispute.tx, TransactionStatus::Disputed)
                .await?;
            self.clients
                .update(
                    &dispute.client,
                    ClientUpdate::Dispute {
                        available_decrease: value.clone(),
                        held_increase: value,
                    },
                )
                .await?;
        }
        Ok(())
    }

    async fn process_resolve(&mut self, resolve: Resolve) -> EngineResult {
        let state = self
            .transactions
            .get_transaction_status(&resolve.tx)
            .await?;
        let amount = self.transactions.get_transaction_value(&resolve.tx).await?;

        // only process resolution if transaction is in a disputed state
        if state == TransactionStatus::Disputed {
            self.transactions
                .store_transaction_status(resolve.tx, TransactionStatus::Resolved)
                .await?;
            self.clients
                .update(
                    &resolve.client,
                    ClientUpdate::Resolve {
                        available_increase: amount.clone(),
                        held_decrease: amount.clone(),
                    },
                )
                .await?;
        }
        Ok(())
    }

    async fn process_chargeback(&mut self, chargeback: Chargeback) -> EngineResult {
        todo!()
    }
}

#[cfg(test)]
mod tests;

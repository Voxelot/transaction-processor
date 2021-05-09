use crate::domain::model::{Client, Transaction};
use crate::domain::ports::{Engine, EngineConfig, EngineError};
use async_trait::async_trait;
use futures::prelude::stream::BoxStream;
use futures::Stream;
use std::marker::PhantomData;

pub struct TransactionEngine<T: EngineConfig> {
    clients: T::ClientRepository,
    transactions: T::TransactionRepository,
}

#[async_trait]
impl<T> Engine for TransactionEngine<T>
where
    T: EngineConfig,
{
    async fn process_transaction(&mut self, transaction: Transaction) {
        todo!()
    }

    async fn get_clients(
        &self,
    ) -> Result<BoxStream<'static, Result<Client, EngineError>>, EngineError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::engine::TransactionEngine;

    mod deposit {
        #[tokio::test]
        async fn deposit_increases_client_available_funds() {}
    }

    mod withdrawal {
        #[tokio::test]
        async fn withdrawal_decreases_client_available_funds() {}

        #[tokio::test]
        async fn withdrawal_rejected_when_available_funds_too_low() {}
    }

    mod dispute {

        #[tokio::test]
        async fn dispute_reduces_available_funds() {}

        #[tokio::test]
        async fn dispute_increases_held_funds() {}

        #[tokio::test]
        async fn dispute_does_not_change_total_funds() {}

        #[tokio::test]
        async fn dispute_does_nothing_when_deposit_already_disputed() {}
    }

    mod resolve {
        #[tokio::test]
        async fn resolve_increases_available_funds_by_disputed_amount() {}

        #[tokio::test]
        async fn resolve_decreases_held_funds_by_disputed_amount() {}

        #[tokio::test]
        async fn resolve_does_nothing_when_transaction_not_disputed() {}

        #[tokio::test]
        async fn resolve_does_nothing_when_dispute_already_resolved() {}

        #[tokio::test]
        async fn resolve_does_nothing_when_chargeback_already_processed() {}
    }

    mod chargeback {
        #[tokio::test]
        async fn chargeback_reduces_held_funds_by_disputed_amount() {}

        #[tokio::test]
        async fn chargeback_reduces_total_funds_by_disputed_amount() {}

        #[tokio::test]
        async fn chargeback_locks_client_account() {}

        #[tokio::test]
        async fn chargeback_does_nothing_if_transaction_not_disputed() {}

        #[tokio::test]
        async fn chargeback_does_nothing_if_transaction_already_charged_back() {}

        #[tokio::test]
        async fn chargeback_does_nothing_if_transaction_already_resolved() {}
    }

    mod test_helpers {
        use crate::domain::model::{
            Client, ClientId, Transaction, TransactionId, TransactionStatus,
        };
        use crate::domain::ports::{
            ClientRepository, ClientRepositoryErrors, TransactionRepositoryErrors,
            TransactionsRepository,
        };
        use async_trait::async_trait;
        use futures::prelude::stream::BoxStream;
        use futures::stream::{self, StreamExt};
        use std::collections::HashMap;

        struct FakeClientRepository {
            clients: HashMap<ClientId, Client>,
        }

        #[async_trait]
        impl ClientRepository for FakeClientRepository {
            async fn get_clients(
                &self,
            ) -> Result<
                BoxStream<'static, Result<Client, ClientRepositoryErrors>>,
                ClientRepositoryErrors,
            > {
                let clients: Vec<Client> = self.clients.values().cloned().collect();
                let stream = stream::iter(clients.into_iter().map(Ok));
                Ok(stream.boxed())
            }

            async fn update_client(
                &mut self,
                client: Client,
            ) -> Result<(), ClientRepositoryErrors> {
                let _ = self.clients.insert(client.id, client).unwrap();
                Ok(())
            }
        }

        struct FakeTransactionRepository {
            transactions: HashMap<TransactionId, TransactionStatus>,
        }

        #[async_trait]
        impl TransactionsRepository for FakeTransactionRepository {
            async fn get_transaction_status(
                &self,
                transaction_id: &TransactionId,
            ) -> Result<TransactionStatus, TransactionRepositoryErrors> {
                self.transactions
                    .get(transaction_id)
                    .cloned()
                    .ok_or_else(|| {
                        TransactionRepositoryErrors::TransactionNotFound(transaction_id.clone())
                    })
            }

            async fn store_transaction_status(
                &mut self,
                transaction_id: TransactionId,
                transaction_status: TransactionStatus,
            ) -> Result<(), TransactionRepositoryErrors> {
                let _ = self.transactions.insert(transaction_id, transaction_status);
                Ok(())
            }
        }
    }
}

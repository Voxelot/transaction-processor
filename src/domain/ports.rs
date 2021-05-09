use crate::domain::model::{Client, Transaction, TransactionId, TransactionStatus};
use async_trait::async_trait;
use futures::prelude::stream::BoxStream;
use futures::Stream;
use thiserror::Error;

#[async_trait]
pub trait Engine {
    async fn process_transaction(&mut self, transaction: Transaction) -> Result<(), EngineError>;
    async fn get_clients(
        &self,
    ) -> Result<BoxStream<'static, Result<Client, EngineError>>, EngineError>;
}

#[derive(Error, Debug)]
pub enum EngineError {
    #[error(transparent)]
    FailedClientFetch(#[from] anyhow::Error),
}

/// Use associated types to wrap generic constraints for dependency injection
pub trait EngineConfig {
    type ClientRepository: ClientRepository + Send + Sync;
    type TransactionRepository: TransactionsRepository + Send + Sync;
}

#[async_trait]
pub trait ClientRepository {
    async fn get_clients(
        &self,
    ) -> Result<BoxStream<'static, Result<Client, ClientRepositoryErrors>>, ClientRepositoryErrors>;

    async fn update_client(&mut self, client: Client) -> Result<(), ClientRepositoryErrors>;
}

#[derive(Error, Debug)]
pub enum ClientRepositoryErrors {
    // used to capture errors such as connectivity issues with a database
    #[error(transparent)]
    AdapterError(#[from] anyhow::Error),
}

#[async_trait]
pub trait TransactionsRepository {
    async fn get_transaction_status(
        &self,
        transaction_id: &TransactionId,
    ) -> Result<TransactionStatus, TransactionRepositoryErrors>;

    async fn store_transaction_status(
        &mut self,
        transaction_id: TransactionId,
        transaction_status: TransactionStatus,
    ) -> Result<(), TransactionRepositoryErrors>;
}

#[derive(Error, Debug)]
pub enum TransactionRepositoryErrors {
    #[error("Transaction already exists {0:?}")]
    DuplicateTransaction(TransactionId),
    #[error("Transaction not found {0:?}")]
    TransactionNotFound(TransactionId),
    // used to capture errors such as connectivity issues with a database
    #[error(transparent)]
    AdapterError(#[from] anyhow::Error),
}

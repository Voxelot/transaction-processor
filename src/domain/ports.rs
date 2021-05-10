use crate::domain::model::{
    AmountInMinorUnits, Client, ClientId, Transaction, TransactionId, TransactionStatus,
};
use async_trait::async_trait;
use futures::prelude::stream::BoxStream;
use thiserror::Error;

pub type EngineResult = Result<(), EngineErrors>;

#[async_trait]
pub trait Engine {
    async fn process_transaction(&mut self, transaction: Transaction) -> EngineResult;
    async fn get_clients(
        &self,
    ) -> Result<BoxStream<'static, Result<Client, EngineErrors>>, EngineErrors>;
}

#[derive(Error, Debug)]
pub enum EngineErrors {
    #[error(transparent)]
    ClientError(#[from] ClientRepositoryErrors),
    #[error(transparent)]
    TransactionError(#[from] TransactionRepositoryErrors),
}

/// Use associated types to wrap generic constraints for dependency injection
pub trait EngineConfig {
    type ClientRepository: ClientRepository + Send + Sync;
    type TransactionRepository: TransactionsRepository + Send + Sync;
}

#[async_trait]
pub trait ClientRepository {
    async fn get_all(
        &self,
    ) -> Result<BoxStream<'static, Result<Client, ClientRepositoryErrors>>, ClientRepositoryErrors>;

    async fn get(&self, client_id: &ClientId) -> Result<Client, ClientRepositoryErrors>;

    async fn insert(&mut self, client: Client) -> Result<(), ClientRepositoryErrors>;

    async fn update(
        &mut self,
        id: &ClientId,
        update: ClientUpdate,
    ) -> Result<(), ClientRepositoryErrors>;
}

pub enum ClientUpdate {
    Deposit {
        available_increase: AmountInMinorUnits,
        total_increase: AmountInMinorUnits,
    },
    Withdrawal {
        available_decrease: AmountInMinorUnits,
        total_decrease: AmountInMinorUnits,
    },
    Dispute {
        available_decrease: AmountInMinorUnits,
        held_increase: AmountInMinorUnits,
    },
    Resolve {
        available_increase: AmountInMinorUnits,
        held_decrease: AmountInMinorUnits,
    },
    Chargeback {
        held_decrease: AmountInMinorUnits,
        total_decrease: AmountInMinorUnits,
    },
}

#[derive(Error, Debug)]
pub enum ClientRepositoryErrors {
    // used to capture errors such as connectivity issues with a database
    #[error(transparent)]
    AdapterError(#[from] anyhow::Error),
    #[error("client not found with id {0:?}")]
    ClientNotFound(ClientId),
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

    async fn get_transaction_value(
        &self,
        transaction_id: &TransactionId,
    ) -> Result<AmountInMinorUnits, TransactionRepositoryErrors>;

    async fn store_transaction_value(
        &mut self,
        transaction_id: TransactionId,
        amount: AmountInMinorUnits,
    ) -> Result<(), TransactionRepositoryErrors>;
}

#[derive(Error, Debug)]
pub enum TransactionRepositoryErrors {
    #[error("Transaction not found {0:?}")]
    TransactionNotFound(TransactionId),
    // used to capture errors such as connectivity issues with a database
    #[error(transparent)]
    AdapterError(#[from] anyhow::Error),
}

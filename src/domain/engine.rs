use crate::domain::model::{Client, Transaction};
use crate::domain::ports::{Engine, EngineConfig, EngineError};
use async_trait::async_trait;
use futures::prelude::stream::BoxStream;
use futures::Stream;
use std::marker::PhantomData;

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
    async fn process_transaction(&mut self, transaction: Transaction) -> Result<(), EngineError> {
        todo!()
    }

    async fn get_clients(
        &self,
    ) -> Result<BoxStream<'static, Result<Client, EngineError>>, EngineError> {
        todo!()
    }
}

#[cfg(test)]
mod tests;

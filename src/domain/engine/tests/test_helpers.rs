use crate::domain::engine::TransactionEngine;
use crate::domain::model::{Client, ClientId, Transaction, TransactionId, TransactionStatus};
use crate::domain::ports::{
    ClientRepository, ClientRepositoryErrors, Engine, EngineConfig, EngineError,
    TransactionRepositoryErrors, TransactionsRepository,
};
use async_trait::async_trait;
use futures::prelude::stream::BoxStream;
use futures::stream::{self, StreamExt};
use futures::TryStreamExt;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub const TEST_CLIENT_ID: ClientId = ClientId(1);
pub const TEST_TRANSACTION_ID_1: TransactionId = TransactionId(1);
pub const TEST_TRANSACTION_ID_2: TransactionId = TransactionId(2);

#[derive(Default)]
pub struct TestEngineDeps;

impl EngineConfig for TestEngineDeps {
    type ClientRepository = FakeClientRepository;
    type TransactionRepository = FakeTransactionRepository;
}

pub struct TestContext {
    engine: TransactionEngine<TestEngineDeps>,
    client_repo: FakeClientRepository,
    transaction_repo: FakeTransactionRepository,
}

impl TestContext {
    pub fn new() -> Self {
        let client_repo = FakeClientRepository::default();
        let transaction_repo = FakeTransactionRepository::default();

        let engine = TransactionEngine {
            clients: client_repo.clone(),
            transactions: transaction_repo.clone(),
        };

        Self {
            engine,
            client_repo,
            transaction_repo,
        }
    }

    pub async fn process_transaction(
        &mut self,
        transaction: Transaction,
    ) -> Result<(), EngineError> {
        self.engine.process_transaction(transaction).await
    }

    pub async fn get_clients(&self) -> Vec<Client> {
        self.engine
            .get_clients()
            .await
            .unwrap()
            .try_collect()
            .await
            .unwrap()
    }
}

#[derive(Clone, Default)]
pub struct FakeClientRepository(Arc<RwLock<FakeInnerClientRepository>>);

#[derive(Default)]
struct FakeInnerClientRepository {
    clients: HashMap<ClientId, Client>,
}

#[async_trait]
impl ClientRepository for FakeClientRepository {
    async fn get_clients(
        &self,
    ) -> Result<BoxStream<'static, Result<Client, ClientRepositoryErrors>>, ClientRepositoryErrors>
    {
        self.0.read().unwrap().get_clients()
    }

    async fn update_client(&mut self, client: Client) -> Result<(), ClientRepositoryErrors> {
        self.0.write().unwrap().update_client(client)
    }
}

impl FakeInnerClientRepository {
    fn get_clients(
        &self,
    ) -> Result<BoxStream<'static, Result<Client, ClientRepositoryErrors>>, ClientRepositoryErrors>
    {
        let clients: Vec<Client> = self.clients.values().cloned().collect();
        let stream = stream::iter(clients.into_iter().map(Ok));
        Ok(stream.boxed())
    }

    fn update_client(&mut self, client: Client) -> Result<(), ClientRepositoryErrors> {
        let _ = self.clients.insert(client.id, client).unwrap();
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct FakeTransactionRepository(Arc<RwLock<FakeInnerTransactionRepository>>);

#[derive(Default)]
struct FakeInnerTransactionRepository {
    transactions: HashMap<TransactionId, TransactionStatus>,
}

#[async_trait]
impl TransactionsRepository for FakeTransactionRepository {
    async fn get_transaction_status(
        &self,
        transaction_id: &TransactionId,
    ) -> Result<TransactionStatus, TransactionRepositoryErrors> {
        self.0
            .read()
            .unwrap()
            .get_transaction_status(transaction_id)
    }

    async fn store_transaction_status(
        &mut self,
        transaction_id: TransactionId,
        transaction_status: TransactionStatus,
    ) -> Result<(), TransactionRepositoryErrors> {
        self.0
            .write()
            .unwrap()
            .store_transaction_status(transaction_id, transaction_status)
    }
}

impl FakeInnerTransactionRepository {
    fn get_transaction_status(
        &self,
        transaction_id: &TransactionId,
    ) -> Result<TransactionStatus, TransactionRepositoryErrors> {
        self.transactions
            .get(transaction_id)
            .cloned()
            .ok_or_else(|| TransactionRepositoryErrors::TransactionNotFound(transaction_id.clone()))
    }

    fn store_transaction_status(
        &mut self,
        transaction_id: TransactionId,
        transaction_status: TransactionStatus,
    ) -> Result<(), TransactionRepositoryErrors> {
        let _ = self.transactions.insert(transaction_id, transaction_status);
        Ok(())
    }
}

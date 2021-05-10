use crate::domain::engine::TransactionEngine;
use crate::domain::model::{
    AmountInMinorUnits, Client, ClientId, TransactionId, TransactionStatus,
};
use crate::domain::ports::{
    ClientRepository, ClientRepositoryErrors, Engine, EngineConfig, TransactionRepositoryErrors,
    TransactionsRepository,
};
use async_trait::async_trait;
use futures::prelude::stream::BoxStream;
use futures::stream::{self, StreamExt};
use futures::TryStreamExt;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub const TEST_CLIENT_ID: ClientId = ClientId(1);
pub const TEST_TRANSACTION_ID_1: TransactionId = TransactionId(1);

pub fn test_client(amount: AmountInMinorUnits) -> Client {
    Client {
        id: TEST_CLIENT_ID,
        available: amount.clone(),
        held: AmountInMinorUnits::from(0),
        total: amount,
        locked: false,
    }
}

#[derive(Default)]
pub struct TestEngineDeps;

impl EngineConfig for TestEngineDeps {
    type ClientRepository = FakeClientRepository;
    type TransactionRepository = FakeTransactionRepository;
}

pub struct TestContext {
    pub engine: TransactionEngine<TestEngineDeps>,
    pub client_repo: FakeClientRepository,
    pub transaction_repo: FakeTransactionRepository,
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

    pub async fn get_clients(&self) -> Vec<Client> {
        self.engine
            .get_clients()
            .await
            .unwrap()
            .try_collect()
            .await
            .unwrap()
    }

    pub async fn with_deposit(&mut self, amount: AmountInMinorUnits, held: AmountInMinorUnits) {
        self.client_repo
            .update_client(Client {
                id: TEST_CLIENT_ID,
                available: amount.clone(),
                held: held.clone(),
                total: amount.clone() + held,
                locked: false,
            })
            .await
            .unwrap();

        self.transaction_repo
            .store_transaction_value(TEST_TRANSACTION_ID_1, amount.clone())
            .await
            .unwrap();
        self.transaction_repo
            .store_transaction_status(TEST_TRANSACTION_ID_1, TransactionStatus::Processed)
            .await
            .unwrap();
    }

    /// sets up the test context with a pre-existing deposit & dispute,
    /// useful for testing resolve & chargeback transactions
    pub async fn with_disputed_amount(
        &mut self,
        available_amount: AmountInMinorUnits,
        disputed_amount: AmountInMinorUnits,
    ) {
        self.client_repo
            .update_client(Client {
                id: TEST_CLIENT_ID,
                available: available_amount.clone(),
                held: disputed_amount.clone(),
                total: available_amount + disputed_amount.clone(),
                locked: false,
            })
            .await
            .unwrap();

        self.transaction_repo
            .store_transaction_value(TEST_TRANSACTION_ID_1, disputed_amount.clone())
            .await
            .unwrap();
        self.transaction_repo
            .store_transaction_status(TEST_TRANSACTION_ID_1, TransactionStatus::Disputed)
            .await
            .unwrap();
    }

    pub async fn with_chargeback(
        &mut self,
        chargeback_amount: AmountInMinorUnits,
        available_amount: AmountInMinorUnits,
        held_amount: AmountInMinorUnits,
    ) {
        self.client_repo
            .update_client(Client {
                id: TEST_CLIENT_ID,
                available: available_amount.clone(),
                held: held_amount.clone(),
                total: available_amount + held_amount.clone(),
                locked: true,
            })
            .await
            .unwrap();

        self.transaction_repo
            .store_transaction_value(TEST_TRANSACTION_ID_1, chargeback_amount.clone())
            .await
            .unwrap();
        self.transaction_repo
            .store_transaction_status(TEST_TRANSACTION_ID_1, TransactionStatus::ChargedBack)
            .await
            .unwrap();
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
        let _ = self.clients.insert(client.id, client);
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct FakeTransactionRepository(Arc<RwLock<FakeInnerTransactionRepository>>);

#[derive(Default)]
struct FakeInnerTransactionRepository {
    transaction_status: HashMap<TransactionId, TransactionStatus>,
    transaction_value: HashMap<TransactionId, AmountInMinorUnits>,
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

    async fn get_transaction_value(
        &self,
        transaction_id: &TransactionId,
    ) -> Result<AmountInMinorUnits, TransactionRepositoryErrors> {
        self.0.read().unwrap().get_transaction_value(transaction_id)
    }

    async fn store_transaction_value(
        &mut self,
        transaction_id: TransactionId,
        amount: AmountInMinorUnits,
    ) -> Result<(), TransactionRepositoryErrors> {
        self.0
            .write()
            .unwrap()
            .store_transaction_value(transaction_id, amount)
    }
}

impl FakeInnerTransactionRepository {
    fn get_transaction_status(
        &self,
        transaction_id: &TransactionId,
    ) -> Result<TransactionStatus, TransactionRepositoryErrors> {
        self.transaction_status
            .get(transaction_id)
            .cloned()
            .ok_or_else(|| TransactionRepositoryErrors::TransactionNotFound(transaction_id.clone()))
    }

    fn store_transaction_status(
        &mut self,
        transaction_id: TransactionId,
        transaction_status: TransactionStatus,
    ) -> Result<(), TransactionRepositoryErrors> {
        let _ = self
            .transaction_status
            .insert(transaction_id, transaction_status);
        Ok(())
    }

    fn store_transaction_value(
        &mut self,
        transaction_id: TransactionId,
        amount: AmountInMinorUnits,
    ) -> Result<(), TransactionRepositoryErrors> {
        let _ = self.transaction_value.insert(transaction_id, amount);
        Ok(())
    }

    fn get_transaction_value(
        &self,
        transaction_id: &TransactionId,
    ) -> Result<AmountInMinorUnits, TransactionRepositoryErrors> {
        self.transaction_value
            .get(transaction_id)
            .cloned()
            .ok_or_else(|| TransactionRepositoryErrors::TransactionNotFound(*transaction_id))
    }
}

use crate::adapters::memory::{FakeClientRepository, FakeTransactionRepository, TestEngineDeps};
use crate::domain::engine::TransactionEngine;
use crate::domain::model::{
    AmountInMinorUnits, Client, ClientId, TransactionId, TransactionStatus,
};
use crate::domain::ports::{ClientRepository, Engine, TransactionsRepository};
use futures::TryStreamExt;

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
            .insert(Client {
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
            .insert(Client {
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
            .insert(Client {
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

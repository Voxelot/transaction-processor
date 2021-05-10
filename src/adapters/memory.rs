use crate::domain::model::{
    AmountInMinorUnits, Client, ClientId, TransactionId, TransactionStatus,
};
use crate::domain::ports::{
    ClientRepository, ClientRepositoryErrors, ClientUpdate, EngineConfig,
    TransactionRepositoryErrors, TransactionsRepository,
};
use async_trait::async_trait;
use futures::prelude::stream::BoxStream;
use futures::stream::{self, StreamExt};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Default)]
pub struct InMemoryEngineDeps;

impl EngineConfig for InMemoryEngineDeps {
    type ClientRepository = InMemoryClientRepository;
    type TransactionRepository = InMemoryTransactionRepository;
}

#[derive(Clone, Default)]
pub struct InMemoryClientRepository(Arc<RwLock<HashMap<ClientId, Client>>>);

#[async_trait]
impl ClientRepository for InMemoryClientRepository {
    async fn get_all(
        &self,
    ) -> Result<BoxStream<'static, Result<Client, ClientRepositoryErrors>>, ClientRepositoryErrors>
    {
        let inner = self.0.read().unwrap();
        let clients: Vec<Client> = inner.values().cloned().collect();
        let stream = stream::iter(clients.into_iter().map(Ok));
        Ok(stream.boxed())
    }

    async fn get(&self, client_id: &ClientId) -> Result<Client, ClientRepositoryErrors> {
        self.0
            .read()
            .unwrap()
            .get(client_id)
            .cloned()
            .ok_or_else(|| ClientRepositoryErrors::ClientNotFound(*client_id))
    }

    async fn insert(&mut self, client: Client) -> Result<(), ClientRepositoryErrors> {
        let mut inner = self.0.write().unwrap();
        let _ = inner.insert(client.id, client);
        Ok(())
    }

    async fn update(
        &mut self,
        id: &ClientId,
        update: ClientUpdate,
    ) -> Result<(), ClientRepositoryErrors> {
        let mut inner = self.0.write().unwrap();
        // insert default client if none exist yet
        if !inner.contains_key(id) {
            inner.insert(*id, Default::default());
        }
        let mut client = inner.get_mut(id).unwrap();
        match update {
            ClientUpdate::Deposit {
                available_increase,
                total_increase,
            } => {
                client.available = client.available.clone() + available_increase;
                client.total = client.total.clone() + total_increase;
            }
            ClientUpdate::Withdrawal {
                available_decrease,
                total_decrease,
            } => {
                client.available = client.available.clone() - available_decrease;
                client.total = client.total.clone() - total_decrease;
            }
            ClientUpdate::Dispute {
                available_decrease,
                held_increase,
            } => {
                client.available = client.available.clone() - available_decrease;
                client.held = client.held.clone() + held_increase;
            }
            ClientUpdate::Resolve {
                available_increase,
                held_decrease,
            } => {
                client.available = client.available.clone() + available_increase;
                client.held = client.held.clone() - held_decrease;
            }
            ClientUpdate::Chargeback {
                held_decrease,
                total_decrease,
            } => {
                client.held = client.held.clone() - held_decrease;
                client.total = client.total.clone() - total_decrease;
                client.locked = true;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct InMemoryTransactionRepository(Arc<RwLock<InnerTransactionRepository>>);

// use inner wrapper so Arc<RwLock<>> can cover multiple hashmaps
#[derive(Default)]
struct InnerTransactionRepository {
    transaction_status: HashMap<TransactionId, TransactionStatus>,
    transaction_value: HashMap<TransactionId, AmountInMinorUnits>,
}

#[async_trait]
impl TransactionsRepository for InMemoryTransactionRepository {
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
            .store_transaction_value(transaction_id, amount);
        Ok(())
    }
}

impl InnerTransactionRepository {
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
    ) {
        let _ = self.transaction_value.insert(transaction_id, amount);
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

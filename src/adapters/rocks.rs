use crate::domain::model::{Client, ClientId};
use crate::domain::ports::{ClientRepository, ClientRepositoryErrors, ClientUpdate};
use bincode::{deserialize, serialize};
use futures::prelude::stream::BoxStream;
use rocksdb::{ColumnFamily, DB};
use std::borrow::BorrowMut;
use std::env::temp_dir;
use std::sync::{Arc, RwLock};

const CLIENT_CF: &str = "clients";

pub struct RocksDb {
    db: Arc<RwLock<DB>>,
}

impl RocksDb {
    fn init() -> Self {
        let db = Arc::new(RwLock::new(DB::open_default("_test_path").unwrap()));
        db.write()
            .unwrap()
            .create_cf(CLIENT_CF, &Default::default())
            .unwrap();

        RocksDb { db }
    }
}

#[async_trait::async_trait]
impl ClientRepository for RocksDb {
    async fn get_all(
        &self,
    ) -> Result<BoxStream<'static, Result<Client, ClientRepositoryErrors>>, ClientRepositoryErrors>
    {
        todo!()
    }

    async fn get(&self, client_id: &ClientId) -> Result<Client, ClientRepositoryErrors> {
        let read_guard = self.db.read().unwrap();
        let client_cf = read_guard
            .cf_handle(CLIENT_CF)
            .expect("Missing column family");

        let client_id_bytes =
            serialize(client_id).map_err(|e| ClientRepositoryErrors::AdapterError(e.into()))?;

        read_guard
            .get_cf(client_cf, client_id_bytes)
            .map_err(|e| ClientRepositoryErrors::AdapterError(e.into()))
            .and_then(|result| {
                let bytes = result
                    .ok_or_else(|| ClientRepositoryErrors::ClientNotFound(client_id.clone()))?;
                let client: Client = bincode::deserialize(bytes.as_slice()).unwrap();
                Ok(client)
            })
    }

    async fn insert(&mut self, client: Client) -> Result<(), ClientRepositoryErrors> {
        let write_guard = self.db.write().unwrap();
        let client_cf = write_guard
            .cf_handle(CLIENT_CF)
            .expect("Missing column family");

        let key =
            serialize(&client.id).map_err(|e| ClientRepositoryErrors::AdapterError(e.into()))?;
        let value =
            serialize(&client).map_err(|e| ClientRepositoryErrors::AdapterError(e.into()))?;

        write_guard
            .put_cf(client_cf, key, value)
            .map_err(|e| ClientRepositoryErrors::AdapterError(e.into()))?;
        Ok(())
    }

    async fn update(
        &mut self,
        id: &ClientId,
        update: ClientUpdate,
    ) -> Result<(), ClientRepositoryErrors> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::adapters::rocks::RocksDb;
    use crate::domain::model::{Client, ClientId};
    use crate::domain::ports::ClientRepository;

    #[tokio::test]
    async fn can_insert_client() {
        let mut rocks = RocksDb::init();
        rocks.insert(Default::default()).await;
        let client = rocks.get(&ClientId(0)).await.unwrap();

        assert_eq!(client, Client::default())
    }

    #[test]
    fn can_roundtrip_client() {
        let client = Client::default();
        let serialized = bincode::serialize(&client).unwrap();
        let deserialize: Client = bincode::deserialize(&serialized).unwrap();
        assert_eq!(client, deserialize);
    }
}

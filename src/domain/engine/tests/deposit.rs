use crate::domain::engine::tests::test_helpers::{
    TestContext, TEST_CLIENT_ID, TEST_TRANSACTION_ID_1,
};
use crate::domain::engine::TransactionEngine;
use crate::domain::model::{AmountInMinorUnits, Client, ClientId, Deposit, Transaction};
use crate::domain::ports::Engine;

#[tokio::test]
async fn deposit_increases_client_available_funds() {
    let mut ctx = TestContext::new();
    ctx.process_transaction(Transaction::Deposit(Deposit {
        client: TEST_CLIENT_ID,
        tx: TEST_TRANSACTION_ID_1,
        amount: AmountInMinorUnits::from(5),
    }));
    let clients = ctx.get_clients().await;

    assert_eq!(
        clients[0],
        Client {
            id: TEST_CLIENT_ID,
            available: AmountInMinorUnits::from(5),
            held: AmountInMinorUnits::from(0),
            total: AmountInMinorUnits::from(5),
            locked: false
        }
    );
}

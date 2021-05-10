use crate::domain::engine::tests::test_helpers::{
    test_client, TestContext, TEST_CLIENT_ID, TEST_TRANSACTION_ID_1,
};
use crate::domain::model::{AmountInMinorUnits, Dispute, Transaction, TransactionStatus};
use crate::domain::ports::{ClientRepository, Engine, TransactionsRepository};

#[tokio::test]
async fn dispute_reduces_available_funds_by_transaction_amount() {
    // test setup
    let mut ctx = TestContext::new();
    ctx.client_repo
        .insert(test_client(AmountInMinorUnits::from(1000)))
        .await
        .unwrap();
    ctx.transaction_repo
        .store_transaction_status(TEST_TRANSACTION_ID_1, TransactionStatus::Processed)
        .await
        .unwrap();
    ctx.transaction_repo
        .store_transaction_value(TEST_TRANSACTION_ID_1, AmountInMinorUnits::from(100))
        .await
        .unwrap();

    // test subject
    ctx.engine
        .process_transaction(Transaction::Dispute(Dispute {
            client: TEST_CLIENT_ID,
            tx: TEST_TRANSACTION_ID_1,
        }))
        .await
        .unwrap();

    // check results
    let clients = ctx.get_clients().await;
    assert_eq!(clients[0].available, AmountInMinorUnits::from(900u64))
}

#[tokio::test]
async fn dispute_increases_held_funds_by_transaction_amount() {
    // test setup
    let mut ctx = TestContext::new();
    ctx.client_repo
        .insert(test_client(AmountInMinorUnits::from(1000)))
        .await
        .unwrap();
    ctx.transaction_repo
        .store_transaction_status(TEST_TRANSACTION_ID_1, TransactionStatus::Processed)
        .await
        .unwrap();
    ctx.transaction_repo
        .store_transaction_value(TEST_TRANSACTION_ID_1, AmountInMinorUnits::from(100))
        .await
        .unwrap();

    // test subject
    ctx.engine
        .process_transaction(Transaction::Dispute(Dispute {
            client: TEST_CLIENT_ID,
            tx: TEST_TRANSACTION_ID_1,
        }))
        .await
        .unwrap();

    // check results
    let clients = ctx.get_clients().await;
    assert_eq!(clients[0].held, AmountInMinorUnits::from(100))
}

#[tokio::test]
async fn dispute_does_not_change_total_funds() {
    // test setup
    let mut ctx = TestContext::new();
    ctx.client_repo
        .insert(test_client(AmountInMinorUnits::from(1000)))
        .await
        .unwrap();
    ctx.transaction_repo
        .store_transaction_status(TEST_TRANSACTION_ID_1, TransactionStatus::Processed)
        .await
        .unwrap();
    ctx.transaction_repo
        .store_transaction_value(TEST_TRANSACTION_ID_1, AmountInMinorUnits::from(100))
        .await
        .unwrap();

    // test subject
    ctx.engine
        .process_transaction(Transaction::Dispute(Dispute {
            client: TEST_CLIENT_ID,
            tx: TEST_TRANSACTION_ID_1,
        }))
        .await
        .unwrap();

    // check results
    let clients = ctx.get_clients().await;
    assert_eq!(clients[0].total, AmountInMinorUnits::from(1000))
}

#[tokio::test]
async fn dispute_changes_transaction_status() {
    // test setup
    let amount_available = AmountInMinorUnits::from(100);
    let amount_held = AmountInMinorUnits::from(50);
    let mut ctx = TestContext::new();
    ctx.with_deposit(amount_available, amount_held).await;

    // test subject
    ctx.engine
        .process_transaction(Transaction::Dispute(Dispute {
            client: TEST_CLIENT_ID,
            tx: TEST_TRANSACTION_ID_1,
        }))
        .await
        .unwrap();

    // check results
    let status = ctx
        .transaction_repo
        .get_transaction_status(&TEST_TRANSACTION_ID_1)
        .await
        .unwrap();
    assert_eq!(status, TransactionStatus::Disputed)
}

#[tokio::test]
async fn dispute_does_not_change_available_funds_if_txn_already_disputed() {
    // test setup
    let available_amount = AmountInMinorUnits::from(100);
    let disputed_amount = AmountInMinorUnits::from(50);
    let mut ctx = TestContext::new();
    ctx.with_disputed_amount(available_amount.clone(), disputed_amount)
        .await;

    // test subject
    ctx.engine
        .process_transaction(Transaction::Dispute(Dispute {
            client: TEST_CLIENT_ID,
            tx: TEST_TRANSACTION_ID_1,
        }))
        .await
        .unwrap();

    // check results
    let clients = ctx.get_clients().await;
    assert_eq!(clients[0].available, available_amount)
}

#[tokio::test]
async fn dispute_does_not_change_held_funds_if_txn_already_disputed() {
    // test setup

    let available_amount = AmountInMinorUnits::from(100);
    let disputed_amount = AmountInMinorUnits::from(50);
    let mut ctx = TestContext::new();
    ctx.with_disputed_amount(available_amount.clone(), disputed_amount.clone())
        .await;

    // test subject
    ctx.engine
        .process_transaction(Transaction::Dispute(Dispute {
            client: TEST_CLIENT_ID,
            tx: TEST_TRANSACTION_ID_1,
        }))
        .await
        .unwrap();

    // check results
    let clients = ctx.get_clients().await;
    assert_eq!(clients[0].held, disputed_amount)
}

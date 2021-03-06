use crate::domain::engine::tests::test_helpers::{
    TestContext, TEST_CLIENT_ID, TEST_TRANSACTION_ID_1,
};
use crate::domain::model::{AmountInMinorUnits, Deposit, Transaction};
use crate::domain::ports::Engine;

#[tokio::test]
async fn deposit_increases_client_available_funds_by_deposit_amount() {
    // test setup
    let mut ctx = TestContext::new();

    // test subject
    ctx.engine
        .process_transaction(Transaction::Deposit(Deposit {
            client: TEST_CLIENT_ID,
            tx: TEST_TRANSACTION_ID_1,
            amount: AmountInMinorUnits::from(5),
        }))
        .await
        .unwrap();

    // check results
    let clients = ctx.get_clients().await;
    assert_eq!(clients[0].available, AmountInMinorUnits::from(5));
}

#[tokio::test]
async fn deposit_increases_client_total_funds_by_deposit_amount() {
    // test setup
    let mut ctx = TestContext::new();

    // test subject
    ctx.engine
        .process_transaction(Transaction::Deposit(Deposit {
            client: TEST_CLIENT_ID,
            tx: TEST_TRANSACTION_ID_1,
            amount: AmountInMinorUnits::from(5),
        }))
        .await
        .unwrap();

    // check results
    let clients = ctx.get_clients().await;
    assert_eq!(clients[0].total, AmountInMinorUnits::from(5));
}

#[tokio::test]
async fn deposit_does_not_increase_available_funds_if_already_processed() {
    // test setup
    let deposit_amount = AmountInMinorUnits::from(100);
    let mut ctx = TestContext::new();

    ctx.with_deposit(deposit_amount.clone(), AmountInMinorUnits::from(0))
        .await;

    // test subject
    ctx.engine
        .process_transaction(Transaction::Deposit(Deposit {
            client: TEST_CLIENT_ID,
            tx: TEST_TRANSACTION_ID_1,
            amount: deposit_amount.clone(),
        }))
        .await
        .unwrap();

    // check results
    let clients = ctx.get_clients().await;
    assert_eq!(clients[0].available, deposit_amount);
}

#[tokio::test]
async fn deposit_does_not_increase_total_funds_if_already_processed() {
    // test setup
    let deposit_amount = AmountInMinorUnits::from(100);
    let mut ctx = TestContext::new();

    ctx.with_deposit(deposit_amount.clone(), AmountInMinorUnits::from(0))
        .await;

    // test subject
    ctx.engine
        .process_transaction(Transaction::Deposit(Deposit {
            client: TEST_CLIENT_ID,
            tx: TEST_TRANSACTION_ID_1,
            amount: deposit_amount.clone(),
        }))
        .await
        .unwrap();

    // check results
    let clients = ctx.get_clients().await;
    assert_eq!(clients[0].total, deposit_amount);
}

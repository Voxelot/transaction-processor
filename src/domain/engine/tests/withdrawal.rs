use crate::domain::engine::tests::test_helpers::test_client;
use crate::domain::model::Withdrawal;
use crate::domain::ports::ClientRepository;
use crate::{
    domain::engine::tests::test_helpers::{TestContext, TEST_CLIENT_ID, TEST_TRANSACTION_ID_1},
    domain::model::{AmountInMinorUnits, Transaction},
    domain::ports::Engine,
};

#[tokio::test]
async fn successful_withdrawal_decreases_client_available_funds() {
    // test setup
    let mut ctx = TestContext::new();
    ctx.client_repo
        .insert(test_client(AmountInMinorUnits::from(100)))
        .await
        .unwrap();

    // test subject
    ctx.engine
        .process_transaction(Transaction::Withdrawal(Withdrawal {
            client: TEST_CLIENT_ID,
            tx: TEST_TRANSACTION_ID_1,
            amount: AmountInMinorUnits::from(10u64),
        }))
        .await
        .unwrap();

    // check results
    let clients = ctx.get_clients().await;
    assert_eq!(clients[0].available, AmountInMinorUnits::from(90u64))
}

#[tokio::test]
async fn successful_withdrawal_decreases_client_total_funds() {
    // test setup
    let mut ctx = TestContext::new();
    ctx.client_repo
        .insert(test_client(AmountInMinorUnits::from(100)))
        .await
        .unwrap();

    // test subject
    ctx.engine
        .process_transaction(Transaction::Withdrawal(Withdrawal {
            client: TEST_CLIENT_ID,
            tx: TEST_TRANSACTION_ID_1,
            amount: AmountInMinorUnits::from(10u64),
        }))
        .await
        .unwrap();

    // check results
    let clients = ctx.get_clients().await;
    assert_eq!(clients[0].total, AmountInMinorUnits::from(90u64))
}

#[tokio::test]
async fn when_available_funds_are_too_low_withdrawal_does_not_effect_available_funds() {
    // test setup
    let mut ctx = TestContext::new();
    ctx.client_repo
        .insert(test_client(AmountInMinorUnits::from(100)))
        .await
        .unwrap();

    // test subject - attempt to withdraw more than the available amount of funds
    ctx.engine
        .process_transaction(Transaction::Withdrawal(Withdrawal {
            client: TEST_CLIENT_ID,
            tx: TEST_TRANSACTION_ID_1,
            amount: AmountInMinorUnits::from(110u64),
        }))
        .await
        .unwrap();

    // check results
    let clients = ctx.get_clients().await;
    assert_eq!(clients[0].available, AmountInMinorUnits::from(100u64))
}

#[tokio::test]
async fn when_available_funds_are_too_low_withdrawal_does_not_effect_total_funds() {
    // test setup
    let mut ctx = TestContext::new();
    ctx.client_repo
        .insert(test_client(AmountInMinorUnits::from(100)))
        .await
        .unwrap();

    // test subject - attempt to withdraw more than the available amount of funds
    ctx.engine
        .process_transaction(Transaction::Withdrawal(Withdrawal {
            client: TEST_CLIENT_ID,
            tx: TEST_TRANSACTION_ID_1,
            amount: AmountInMinorUnits::from(110u64),
        }))
        .await
        .unwrap();

    // check results
    let clients = ctx.get_clients().await;
    assert_eq!(clients[0].total, AmountInMinorUnits::from(100u64))
}

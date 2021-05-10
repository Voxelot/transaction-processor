use crate::domain::engine::tests::test_helpers::{
    TestContext, TEST_CLIENT_ID, TEST_TRANSACTION_ID_1,
};
use crate::domain::model::{AmountInMinorUnits, Chargeback, Transaction};
use crate::domain::ports::Engine;

#[tokio::test]
async fn chargeback_reduces_held_funds_by_disputed_amount() {
    // test setup
    let starting_available_amount = AmountInMinorUnits::from(100);
    let disputed_amount = AmountInMinorUnits::from(50);
    let mut ctx = TestContext::new();
    ctx.with_disputed_amount(starting_available_amount, disputed_amount)
        .await;

    // test subject
    ctx.engine
        .process_transaction(Transaction::Chargeback(Chargeback {
            client: TEST_CLIENT_ID,
            tx: TEST_TRANSACTION_ID_1,
        }))
        .await
        .unwrap();

    // check results
    let clients = ctx.get_clients().await;
    assert_eq!(clients[0].held, AmountInMinorUnits::from(0));
}

#[tokio::test]
async fn chargeback_reduces_total_funds_by_disputed_amount() {
    // test setup
    let starting_available_amount = AmountInMinorUnits::from(100);
    let disputed_amount = AmountInMinorUnits::from(50);
    let mut ctx = TestContext::new();
    ctx.with_disputed_amount(starting_available_amount.clone(), disputed_amount.clone())
        .await;

    // test subject
    ctx.engine
        .process_transaction(Transaction::Chargeback(Chargeback {
            client: TEST_CLIENT_ID,
            tx: TEST_TRANSACTION_ID_1,
        }))
        .await
        .unwrap();

    // check results
    let clients = ctx.get_clients().await;
    assert_eq!(
        clients[0].total,
        starting_available_amount - disputed_amount
    );
}

#[tokio::test]
async fn chargeback_locks_client_account() {
    // test setup
    let starting_available_amount = AmountInMinorUnits::from(100);
    let disputed_amount = AmountInMinorUnits::from(50);
    let mut ctx = TestContext::new();
    ctx.with_disputed_amount(starting_available_amount.clone(), disputed_amount.clone())
        .await;

    // test subject
    ctx.engine
        .process_transaction(Transaction::Chargeback(Chargeback {
            client: TEST_CLIENT_ID,
            tx: TEST_TRANSACTION_ID_1,
        }))
        .await
        .unwrap();

    // check results
    let clients = ctx.get_clients().await;
    assert_eq!(clients[0].locked, true);
}

#[tokio::test]
async fn chargeback_does_not_decrease_held_funds_if_transaction_not_disputed() {
    // test setup
    let starting_available_amount = AmountInMinorUnits::from(100);
    let held_amount = AmountInMinorUnits::from(20);
    let mut ctx = TestContext::new();
    ctx.with_deposit(starting_available_amount.clone(), held_amount.clone())
        .await;

    // test subject
    ctx.engine
        .process_transaction(Transaction::Chargeback(Chargeback {
            client: TEST_CLIENT_ID,
            tx: TEST_TRANSACTION_ID_1,
        }))
        .await
        .unwrap();

    // check results
    let clients = ctx.get_clients().await;
    assert_eq!(clients[0].held, held_amount);
}

#[tokio::test]
async fn chargeback_does_not_decrease_total_funds_if_transaction_not_disputed() {
    // test setup
    let starting_available_amount = AmountInMinorUnits::from(100);
    let held_amount = AmountInMinorUnits::from(20);
    let mut ctx = TestContext::new();
    ctx.with_deposit(starting_available_amount.clone(), held_amount.clone())
        .await;

    // test subject
    ctx.engine
        .process_transaction(Transaction::Chargeback(Chargeback {
            client: TEST_CLIENT_ID,
            tx: TEST_TRANSACTION_ID_1,
        }))
        .await
        .unwrap();

    // check results
    let clients = ctx.get_clients().await;
    assert_eq!(clients[0].total, starting_available_amount + held_amount);
}

#[tokio::test]
async fn chargeback_does_not_decrease_held_funds_if_transaction_already_charged_back() {
    // test setup
    let available_amount = AmountInMinorUnits::from(100);
    let held_amount = AmountInMinorUnits::from(20);
    let chargeback_amount = AmountInMinorUnits::from(50);
    let mut ctx = TestContext::new();
    ctx.with_chargeback(
        chargeback_amount,
        available_amount.clone(),
        held_amount.clone(),
    )
    .await;

    // test subject
    ctx.engine
        .process_transaction(Transaction::Chargeback(Chargeback {
            client: TEST_CLIENT_ID,
            tx: TEST_TRANSACTION_ID_1,
        }))
        .await
        .unwrap();

    // check results
    let clients = ctx.get_clients().await;
    assert_eq!(clients[0].held, held_amount);
}

#[tokio::test]
async fn chargeback_does_not_decrease_total_funds_if_transaction_already_charged_back() {
    // test setup
    let available_amount = AmountInMinorUnits::from(100);
    let held_amount = AmountInMinorUnits::from(20);
    let chargeback_amount = AmountInMinorUnits::from(50);
    let mut ctx = TestContext::new();
    ctx.with_chargeback(
        chargeback_amount,
        available_amount.clone(),
        held_amount.clone(),
    )
    .await;

    // test subject
    ctx.engine
        .process_transaction(Transaction::Chargeback(Chargeback {
            client: TEST_CLIENT_ID,
            tx: TEST_TRANSACTION_ID_1,
        }))
        .await
        .unwrap();

    // check results
    let clients = ctx.get_clients().await;
    assert_eq!(clients[0].total, available_amount + held_amount);
}

#[tokio::test]
async fn chargeback_does_not_decrease_held_funds_if_transaction_already_resolved() {
    // test setup
    let available_amount = AmountInMinorUnits::from(100);
    let held_amount = AmountInMinorUnits::from(20);
    let chargeback_amount = AmountInMinorUnits::from(50);
    let mut ctx = TestContext::new();
    ctx.with_chargeback(
        chargeback_amount,
        available_amount.clone(),
        held_amount.clone(),
    )
    .await;

    // test subject
    ctx.engine
        .process_transaction(Transaction::Chargeback(Chargeback {
            client: TEST_CLIENT_ID,
            tx: TEST_TRANSACTION_ID_1,
        }))
        .await
        .unwrap();

    // check results
    let clients = ctx.get_clients().await;
    assert_eq!(clients[0].held, held_amount);
}

#[tokio::test]
async fn chargeback_does_not_decrease_total_funds_if_transaction_already_resolved() {
    // test setup
    let available_amount = AmountInMinorUnits::from(100);
    let held_amount = AmountInMinorUnits::from(20);
    let chargeback_amount = AmountInMinorUnits::from(50);
    let mut ctx = TestContext::new();
    ctx.with_chargeback(
        chargeback_amount,
        available_amount.clone(),
        held_amount.clone(),
    )
    .await;

    // test subject
    ctx.engine
        .process_transaction(Transaction::Chargeback(Chargeback {
            client: TEST_CLIENT_ID,
            tx: TEST_TRANSACTION_ID_1,
        }))
        .await
        .unwrap();

    // check results
    let clients = ctx.get_clients().await;
    assert_eq!(clients[0].total, available_amount + held_amount);
}

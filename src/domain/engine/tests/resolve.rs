use crate::domain::engine::tests::test_helpers::{
    TestContext, TEST_CLIENT_ID, TEST_TRANSACTION_ID_1,
};
use crate::domain::model::{AmountInMinorUnits, Client, Resolve, Transaction, TransactionStatus};
use crate::domain::ports::{ClientRepository, Engine, TransactionsRepository};

#[tokio::test]
async fn resolve_increases_available_funds_by_disputed_amount() {
    // test setup
    let starting_available_amount = AmountInMinorUnits::from(100);
    let disputed_amount = AmountInMinorUnits::from(300);
    let mut ctx = TestContext::new();
    ctx.with_disputed_amount(starting_available_amount.clone(), disputed_amount.clone())
        .await;

    // test subject
    ctx.engine
        .process_transaction(Transaction::Resolve(Resolve {
            client: TEST_CLIENT_ID,
            tx: TEST_TRANSACTION_ID_1,
        }))
        .await
        .unwrap();

    // check results
    let clients = ctx.get_clients().await;
    assert_eq!(
        clients[0].available,
        starting_available_amount + disputed_amount
    );
}

#[tokio::test]
async fn resolve_decreases_held_funds_by_disputed_amount() {
    // test setup
    let starting_available_amount = AmountInMinorUnits::from(100);
    let disputed_amount = AmountInMinorUnits::from(300);
    let mut ctx = TestContext::new();
    ctx.with_disputed_amount(starting_available_amount.clone(), disputed_amount.clone())
        .await;

    // test subject
    ctx.engine
        .process_transaction(Transaction::Resolve(Resolve {
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
async fn resolve_does_not_affect_available_funds_when_transaction_not_disputed() {
    // test setup
    let starting_available_amount = AmountInMinorUnits::from(100);
    let mut ctx = TestContext::new();
    ctx.client_repo
        .update_client(Client {
            id: TEST_CLIENT_ID,
            available: starting_available_amount.clone(),
            held: AmountInMinorUnits::from(0),
            total: starting_available_amount.clone(),
            locked: false,
        })
        .await
        .unwrap();

    ctx.transaction_repo
        .store_transaction_status(TEST_TRANSACTION_ID_1, TransactionStatus::Processed)
        .await
        .unwrap();
    ctx.transaction_repo
        .store_transaction_value(TEST_TRANSACTION_ID_1, starting_available_amount.clone())
        .await
        .unwrap();

    // test subject
    ctx.engine
        .process_transaction(Transaction::Resolve(Resolve {
            client: TEST_CLIENT_ID,
            tx: TEST_TRANSACTION_ID_1,
        }))
        .await
        .unwrap();

    // check results
    let clients = ctx.get_clients().await;
    assert_eq!(clients[0].available, starting_available_amount);
}

#[tokio::test]
async fn resolve_does_not_affect_held_funds_when_transaction_not_disputed() {
    // test setup
    let starting_available_amount = AmountInMinorUnits::from(100);
    let mut ctx = TestContext::new();
    ctx.client_repo
        .update_client(Client {
            id: TEST_CLIENT_ID,
            available: starting_available_amount.clone(),
            held: AmountInMinorUnits::from(0),
            total: starting_available_amount.clone(),
            locked: false,
        })
        .await
        .unwrap();

    ctx.transaction_repo
        .store_transaction_status(TEST_TRANSACTION_ID_1, TransactionStatus::Processed)
        .await
        .unwrap();
    ctx.transaction_repo
        .store_transaction_value(TEST_TRANSACTION_ID_1, starting_available_amount.clone())
        .await
        .unwrap();

    // test subject
    ctx.engine
        .process_transaction(Transaction::Resolve(Resolve {
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
async fn resolve_does_not_increase_available_funds_when_dispute_already_resolved() {
    // test setup
    let starting_available_amount = AmountInMinorUnits::from(100);
    let mut ctx = TestContext::new();
    ctx.client_repo
        .update_client(Client {
            id: TEST_CLIENT_ID,
            available: starting_available_amount.clone(),
            held: AmountInMinorUnits::from(0),
            total: starting_available_amount.clone(),
            locked: false,
        })
        .await
        .unwrap();

    ctx.transaction_repo
        .store_transaction_status(TEST_TRANSACTION_ID_1, TransactionStatus::Resolved)
        .await
        .unwrap();
    ctx.transaction_repo
        .store_transaction_value(TEST_TRANSACTION_ID_1, starting_available_amount.clone())
        .await
        .unwrap();

    // test subject
    ctx.engine
        .process_transaction(Transaction::Resolve(Resolve {
            client: TEST_CLIENT_ID,
            tx: TEST_TRANSACTION_ID_1,
        }))
        .await
        .unwrap();

    // check results
    let clients = ctx.get_clients().await;
    assert_eq!(clients[0].available, starting_available_amount);
}

#[tokio::test]
async fn resolve_does_not_decrease_held_funds_when_dispute_already_resolved() {
    // test setup
    let starting_available_amount = AmountInMinorUnits::from(100);
    let mut ctx = TestContext::new();
    ctx.client_repo
        .update_client(Client {
            id: TEST_CLIENT_ID,
            available: starting_available_amount.clone(),
            held: AmountInMinorUnits::from(500),
            total: starting_available_amount.clone() + AmountInMinorUnits::from(500),
            locked: false,
        })
        .await
        .unwrap();

    ctx.transaction_repo
        .store_transaction_status(TEST_TRANSACTION_ID_1, TransactionStatus::Resolved)
        .await
        .unwrap();
    ctx.transaction_repo
        .store_transaction_value(TEST_TRANSACTION_ID_1, starting_available_amount.clone())
        .await
        .unwrap();

    // test subject
    ctx.engine
        .process_transaction(Transaction::Resolve(Resolve {
            client: TEST_CLIENT_ID,
            tx: TEST_TRANSACTION_ID_1,
        }))
        .await
        .unwrap();

    // check results
    let clients = ctx.get_clients().await;
    assert_eq!(clients[0].held, AmountInMinorUnits::from(500));
}

#[tokio::test]
async fn resolve_does_not_increase_available_funds_when_already_charged_back() {
    // test setup
    let disputed_amount = AmountInMinorUnits::from(100);
    let mut ctx = TestContext::new();
    ctx.client_repo
        .update_client(Client {
            id: TEST_CLIENT_ID,
            available: AmountInMinorUnits::from(0),
            held: AmountInMinorUnits::from(0),
            total: AmountInMinorUnits::from(0),
            locked: false,
        })
        .await
        .unwrap();

    ctx.transaction_repo
        .store_transaction_status(TEST_TRANSACTION_ID_1, TransactionStatus::ChargedBack)
        .await
        .unwrap();
    ctx.transaction_repo
        .store_transaction_value(TEST_TRANSACTION_ID_1, disputed_amount)
        .await
        .unwrap();

    // test subject
    ctx.engine
        .process_transaction(Transaction::Resolve(Resolve {
            client: TEST_CLIENT_ID,
            tx: TEST_TRANSACTION_ID_1,
        }))
        .await
        .unwrap();

    // check results
    let clients = ctx.get_clients().await;
    assert_eq!(clients[0].available, AmountInMinorUnits::from(0));
}

#[tokio::test]
async fn resolve_does_not_decrease_held_funds_when_already_charged_back() {
    // test setup
    let chargeback_amount = AmountInMinorUnits::from(100);
    let held_amount = AmountInMinorUnits::from(200);
    let mut ctx = TestContext::new();
    ctx.client_repo
        .update_client(Client {
            id: TEST_CLIENT_ID,
            available: AmountInMinorUnits::from(0),
            held: held_amount.clone(),
            total: AmountInMinorUnits::from(200),
            locked: false,
        })
        .await
        .unwrap();

    ctx.transaction_repo
        .store_transaction_status(TEST_TRANSACTION_ID_1, TransactionStatus::ChargedBack)
        .await
        .unwrap();
    ctx.transaction_repo
        .store_transaction_value(TEST_TRANSACTION_ID_1, chargeback_amount)
        .await
        .unwrap();

    // test subject
    ctx.engine
        .process_transaction(Transaction::Resolve(Resolve {
            client: TEST_CLIENT_ID,
            tx: TEST_TRANSACTION_ID_1,
        }))
        .await
        .unwrap();

    // check results
    let clients = ctx.get_clients().await;
    assert_eq!(clients[0].held, held_amount);
}

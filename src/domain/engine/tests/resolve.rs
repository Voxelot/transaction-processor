#[tokio::test]
async fn resolve_increases_available_funds_by_disputed_amount() {}

#[tokio::test]
async fn resolve_decreases_held_funds_by_disputed_amount() {}

#[tokio::test]
async fn resolve_does_nothing_when_transaction_not_disputed() {}

#[tokio::test]
async fn resolve_does_nothing_when_dispute_already_resolved() {}

#[tokio::test]
async fn resolve_does_nothing_when_chargeback_already_processed() {}

use bigdecimal::BigDecimal;

// owned primitives
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ClientId(u16);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct TransactionId(u32);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AmountInMinorUnits(BigDecimal);

// high level objects
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Client {
    pub(crate) id: ClientId,
    available: AmountInMinorUnits,
    held: AmountInMinorUnits,
    total: AmountInMinorUnits,
    locked: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Transaction {
    Deposit(Deposit),
    Withdrawal(Withdrawal),
    Dispute(Dispute),
    Resolve(Resolve),
    Chargeback(Chargeback),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TransactionStatus {
    Processed,
    Disputed,
    Resolved,
    ChargedBack,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Deposit {
    client: ClientId,
    tx: TransactionId,
    amount: AmountInMinorUnits,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Withdrawal {
    client: ClientId,
    tx: TransactionId,
    amount: AmountInMinorUnits,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Dispute {
    client: ClientId,
    tx: TransactionId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Resolve {
    client: ClientId,
    tx: TransactionId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Chargeback {
    client: ClientId,
    tx: TransactionId,
}

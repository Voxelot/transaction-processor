use bigdecimal::BigDecimal;

// owned primitives
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ClientId(pub(crate) u16);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct TransactionId(pub(crate) u32);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AmountInMinorUnits(pub(crate) BigDecimal);

impl From<u64> for AmountInMinorUnits {
    fn from(amount: u64) -> Self {
        AmountInMinorUnits(BigDecimal::from(amount))
    }
}

// high level objects
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Client {
    pub(crate) id: ClientId,
    pub(crate) available: AmountInMinorUnits,
    pub(crate) held: AmountInMinorUnits,
    pub(crate) total: AmountInMinorUnits,
    pub(crate) locked: bool,
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
    pub(crate) client: ClientId,
    pub(crate) tx: TransactionId,
    pub(crate) amount: AmountInMinorUnits,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Withdrawal {
    pub(crate) client: ClientId,
    pub(crate) tx: TransactionId,
    pub(crate) amount: AmountInMinorUnits,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Dispute {
    pub(crate) client: ClientId,
    pub(crate) tx: TransactionId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Resolve {
    pub(crate) client: ClientId,
    pub(crate) tx: TransactionId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Chargeback {
    pub(crate) client: ClientId,
    pub(crate) tx: TransactionId,
}

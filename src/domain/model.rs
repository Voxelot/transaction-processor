use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::ops::{Add, Sub};
use std::str::FromStr;

// owned primitives
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ClientId(pub(crate) u16);

impl FromStr for ClientId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ClientId(s.parse().map_err(|_| ())?))
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct TransactionId(pub(crate) u32);

impl FromStr for TransactionId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TransactionId(s.parse().map_err(|_| ())?))
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct AmountInMinorUnits(Decimal);

impl AmountInMinorUnits {
    fn round_to_4_decimals(self) -> Self {
        AmountInMinorUnits(self.0.round_dp(4))
    }
}

impl From<u64> for AmountInMinorUnits {
    fn from(amount: u64) -> Self {
        AmountInMinorUnits(Decimal::from(amount))
    }
}

impl FromStr for AmountInMinorUnits {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let decimal: Decimal = s.parse().map_err(|_| ())?;
        Ok(AmountInMinorUnits(decimal).round_to_4_decimals())
    }
}

impl Add for AmountInMinorUnits {
    type Output = AmountInMinorUnits;

    fn add(self, rhs: Self) -> Self::Output {
        AmountInMinorUnits(self.0.add(rhs.0)).round_to_4_decimals()
    }
}

impl Sub for AmountInMinorUnits {
    type Output = AmountInMinorUnits;

    fn sub(self, rhs: Self) -> Self::Output {
        AmountInMinorUnits(self.0.sub(rhs.0)).round_to_4_decimals()
    }
}

// high level objects
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Client {
    #[serde(rename = "client")]
    pub(crate) id: ClientId,
    pub(crate) available: AmountInMinorUnits,
    pub(crate) held: AmountInMinorUnits,
    pub(crate) total: AmountInMinorUnits,
    pub(crate) locked: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Transaction {
    Deposit(Deposit),
    Withdrawal(Withdrawal),
    Dispute(Dispute),
    Resolve(Resolve),
    Chargeback(Chargeback),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum TransactionStatus {
    Processed,
    Disputed,
    Resolved,
    ChargedBack,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Deposit {
    pub(crate) client: ClientId,
    pub(crate) tx: TransactionId,
    pub(crate) amount: AmountInMinorUnits,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Withdrawal {
    pub(crate) client: ClientId,
    pub(crate) tx: TransactionId,
    pub(crate) amount: AmountInMinorUnits,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Dispute {
    pub(crate) client: ClientId,
    pub(crate) tx: TransactionId,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Resolve {
    pub(crate) client: ClientId,
    pub(crate) tx: TransactionId,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Chargeback {
    pub(crate) client: ClientId,
    pub(crate) tx: TransactionId,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputRecord {
    #[serde(rename = "type")]
    pub(crate) tx_type: String,
    pub(crate) client: String,
    pub(crate) tx: String,
    pub(crate) amount: Option<String>,
}

impl TryFrom<InputRecord> for Transaction {
    type Error = ();

    fn try_from(value: InputRecord) -> Result<Self, Self::Error> {
        // TODO: investigate using strum to convert from string to enum variant
        let transaction = match value.tx_type.as_str() {
            "deposit" => Transaction::Deposit(Deposit {
                client: ClientId::from_str(value.client.as_str()).map_err(|_| ())?,
                tx: TransactionId::from_str(value.tx.as_str()).map_err(|_| ())?,
                amount: AmountInMinorUnits::from_str(value.amount.ok_or(())?.as_str())?,
            }),
            "withdrawal" => Transaction::Withdrawal(Withdrawal {
                client: ClientId::from_str(value.client.as_str()).map_err(|_| ())?,
                tx: TransactionId::from_str(value.tx.as_str()).map_err(|_| ())?,
                amount: AmountInMinorUnits::from_str(value.amount.ok_or(())?.as_str())?,
            }),
            "dispute" => Transaction::Dispute(Dispute {
                client: ClientId::from_str(value.client.as_str()).map_err(|_| ())?,
                tx: TransactionId::from_str(value.tx.as_str()).map_err(|_| ())?,
            }),
            "resolve" => Transaction::Resolve(Resolve {
                client: ClientId::from_str(value.client.as_str()).map_err(|_| ())?,
                tx: TransactionId::from_str(value.tx.as_str()).map_err(|_| ())?,
            }),
            _ => return Err(()),
        };
        Ok(transaction)
    }
}

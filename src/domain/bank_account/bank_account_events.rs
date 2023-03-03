use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BankAccountEvent {
    AccountOpened {
        account_id: String,
    },
    CustomerDepositedMoney {
        amount: f64,
        balance: f64,
    },
    CustomerWithdrewCash {
        amount: f64,
        balance: f64,
    },
    CustomerWroteCheck {
        check_number: String,
        amount: f64,
        balance: f64,
    },
}

impl DomainEvent for BankAccountEvent {
    fn event_type(&self) -> String {
        match self {
            BankAccountEvent::AccountOpened { .. } => "AccountOpened".to_string(),
            BankAccountEvent::CustomerDepositedMoney { .. } => "CustomerDepositedMoney".to_string(),
            BankAccountEvent::CustomerWithdrewCash { .. } => "CustomerWithdrewCash".to_string(),
            BankAccountEvent::CustomerWroteCheck { .. } => "CustomerWroteCheck".to_string(),
        }
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum BankAccountError {
    #[error("account already open")]
    AccountAlreadyOpen,

    #[error("cannot deposit negative amount")]
    CannotDepositNegativeAmount,

    #[error("cannot withdraw negative amount")]
    CannotWithdrawNegativeAmount,

    #[error("insufficient funds")]
    InsufficientFunds,

    #[error("invalid amount")]
    InvalidAmount,

    #[error("invalid check number")]
    InvalidCheckNumber,

    #[error("invalid account id")]
    InvalidAccountId,

    #[error("invalid check")]
    InvalidCheck,

    #[error("atm rule violation")]
    AtmRuleViolation,

    #[error("cannot write negative check amount")]
    CannotWriteNegativeCheckAmount,

    #[error("Unexpected Error: {0}")]
    UnexpectedError(String),
}

impl From<&str> for BankAccountError {
    fn from(msg: &str) -> Self {
        match msg {
            "AccountAlreadyOpen" => BankAccountError::AccountAlreadyOpen,
            "CannotDepositNegativeAmount" => BankAccountError::CannotDepositNegativeAmount,
            "InsufficientFunds" => BankAccountError::InsufficientFunds,
            "InvalidAmount" => BankAccountError::InvalidAmount,
            "InvalidCheckNumber" => BankAccountError::InvalidCheckNumber,
            "InvalidAccountId" => BankAccountError::InvalidAccountId,
            "InvalidCheck" => BankAccountError::InvalidCheck,
            "AtmRuleViolation" => BankAccountError::AtmRuleViolation,
            "CannotWriteNegativeCheckAmount" => BankAccountError::CannotWriteNegativeCheckAmount,
            _ => BankAccountError::UnexpectedError(msg.to_string()),
        }
    }
}

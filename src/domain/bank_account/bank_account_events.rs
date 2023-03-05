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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn bank_account_event_version_is_1_0() {
        let event = BankAccountEvent::AccountOpened {
            account_id: "123".to_string(),
        };
        assert_eq!(event.event_version(), "1.0".to_string());
    }

    #[test]
    fn bank_account_event_type_is_account_opened() {
        let event = BankAccountEvent::AccountOpened {
            account_id: "123".to_string(),
        };
        assert_eq!(event.event_type(), "AccountOpened".to_string());
    }

    #[test]
    fn bank_account_event_type_is_customer_deposited_money() {
        let event = BankAccountEvent::CustomerDepositedMoney {
            amount: 100.0,
            balance: 100.0,
        };
        assert_eq!(event.event_type(), "CustomerDepositedMoney".to_string());
    }

    #[test]
    fn bank_account_event_type_is_customer_withdrew_cash() {
        let event = BankAccountEvent::CustomerWithdrewCash {
            amount: 100.0,
            balance: 100.0,
        };
        assert_eq!(event.event_type(), "CustomerWithdrewCash".to_string());
    }

    #[test]
    fn bank_account_event_type_is_customer_wrote_check() {
        let event = BankAccountEvent::CustomerWroteCheck {
            check_number: "123".to_string(),
            amount: 100.0,
            balance: 100.0,
        };
        assert_eq!(event.event_type(), "CustomerWroteCheck".to_string());
    }
}

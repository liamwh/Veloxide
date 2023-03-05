use std::fmt::Debug;

#[derive(thiserror::Error, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use coverage_helper::test;
    use pretty_assertions::assert_eq;

    #[test]
    fn can_create_bank_account_error_from_string() {
        let error = BankAccountError::from("AccountAlreadyOpen");
        assert_eq!(error, BankAccountError::AccountAlreadyOpen);

        let error = BankAccountError::from("CannotDepositNegativeAmount");
        assert_eq!(error, BankAccountError::CannotDepositNegativeAmount);

        let error = BankAccountError::from("InsufficientFunds");
        assert_eq!(error, BankAccountError::InsufficientFunds);

        let error = BankAccountError::from("InvalidAmount");
        assert_eq!(error, BankAccountError::InvalidAmount);

        let error = BankAccountError::from("InvalidCheckNumber");
        assert_eq!(error, BankAccountError::InvalidCheckNumber);

        let error = BankAccountError::from("InvalidAccountId");
        assert_eq!(error, BankAccountError::InvalidAccountId);

        let error = BankAccountError::from("InvalidCheck");
        assert_eq!(error, BankAccountError::InvalidCheck);

        let error = BankAccountError::from("AtmRuleViolation");
        assert_eq!(error, BankAccountError::AtmRuleViolation);

        let error = BankAccountError::from("CannotWriteNegativeCheckAmount");
        assert_eq!(error, BankAccountError::CannotWriteNegativeCheckAmount);

        let error = BankAccountError::from("AnyNonMatchingErrorString");
        assert_eq!(
            error,
            BankAccountError::UnexpectedError("AnyNonMatchingErrorString".to_string())
        );
    }
}

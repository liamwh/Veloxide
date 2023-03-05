#[cfg(test)]
use mockall::{automock, predicate::*};

use async_trait::async_trait;
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct BankAccountServices {
    #[derivative(Debug = "ignore")]
    pub services: Box<dyn BankAccountApi>,
}

#[cfg_attr(test, automock)]
impl BankAccountServices {
    pub fn new(services: Box<dyn BankAccountApi>) -> Self {
        Self { services }
    }
}

// External services must be called during the processing of the command.
#[async_trait]
#[cfg_attr(test, automock)]
pub trait BankAccountApi: Sync + Send {
    async fn atm_withdrawal(&self, atm_id: &str, amount: f64) -> Result<(), AtmError>;
    async fn validate_check(&self, account_id: &str, check: &str) -> Result<(), CheckingError>;
}
pub struct AtmError;
pub struct CheckingError;

// A very simple "happy path" set of services that always succeed.
pub struct HappyPathBankAccountServices;

#[async_trait]
#[cfg_attr(test, automock)]
impl BankAccountApi for HappyPathBankAccountServices {
    async fn atm_withdrawal(&self, _atm_id: &str, _amount: f64) -> Result<(), AtmError> {
        Ok(())
    }

    async fn validate_check(
        &self,
        _account_id: &str,
        _check_number: &str,
    ) -> Result<(), CheckingError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn happy_path_bank_account_services_atm_withdrawal_returns_ok() {
        let services = HappyPathBankAccountServices;
        let result = services.atm_withdrawal("123", 100.0).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn happy_path_bank_account_services_validate_check_returns_ok() {
        let services = HappyPathBankAccountServices;
        let result = services.validate_check("123", "123").await;
        assert!(result.is_ok());
    }
}

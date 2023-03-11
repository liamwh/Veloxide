use async_graphql::*;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, OneofObject, TS)]
#[ts(export, export_to = "frontend/src/bindings/")]
pub enum BankAccountCommand {
    /// OpenAccount
    OpenAccount(BankAccountOpenAccountCommandData),

    /// DepositMoney
    DepositMoney(BankAccountDepositMoneyCommandData),

    /// WithdrawMoney
    WithdrawMoney(BankAccountWithdrawMoneyCommandData),

    /// WriteCheck
    WriteCheck(BankAccountWriteCheckCommandData),
}

#[derive(Debug, Serialize, Deserialize, ToSchema, InputObject, Clone, TS)]
#[ts(export, export_to = "frontend/src/bindings/")]
pub struct BankAccountOpenAccountCommandData {
    pub account_id: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, InputObject, Clone, TS)]
#[ts(export, export_to = "frontend/src/bindings/")]
pub struct BankAccountDepositMoneyCommandData {
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, InputObject, Clone, TS)]
#[ts(export, export_to = "frontend/src/bindings/")]
pub struct BankAccountWithdrawMoneyCommandData {
    pub amount: f64,
    pub atm_id: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, InputObject, Clone, TS)]
#[ts(export, export_to = "frontend/src/bindings/")]
pub struct BankAccountWriteCheckCommandData {
    pub check_number: String,
    pub amount: f64,
}

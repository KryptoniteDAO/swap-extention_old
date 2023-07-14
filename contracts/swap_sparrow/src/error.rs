use cosmwasm_std::{OverflowError, StdError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    OverflowError(#[from] OverflowError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Pair config disabled")]
    PairConfigDisabled {},

    #[error("Balance not enough")]
    BalanceNotEnough {},

    #[error("Pair not found")]
    PairNotFound {},

    #[error("Unable to receive msg")]
    UnableToReceiveMsg {},

    #[error("Invalid denom")]
    InvalidDenom,

    #[error("Invalid parameter")]
    InvalidParameter,

    #[error("Invalid owner")]
    InvalidOwner,

    #[error("Invalid amount")]
    InvalidAmount,
}

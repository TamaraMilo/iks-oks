use std::any::type_name;

use cosmwasm_std::{from_slice, StdError, StdResult};
use serde::de::DeserializeOwned;

pub(crate) fn may_deserialize<T: DeserializeOwned>(
    value: &Option<Vec<u8>>,
) -> StdResult<Option<T>> {
    match value {
        Some(data) => Ok(Some(from_slice(data)?)),
        None => Ok(None),
    }
}

pub(crate) fn must_deserialize<T: DeserializeOwned>(value: &Option<Vec<u8>>) -> StdResult<T> {
    match value {
        Some(data) => from_slice(data),
        None => Err(StdError::not_found(type_name::<T>())),
    }
}

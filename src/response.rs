use std::error::Error;
use serde::de::DeserializeOwned;
use crate::status_code::StatusCode;

pub trait Response {
	type StatusCode: StatusCode;

	fn status(&self) -> Self::StatusCode;

	type JsonError: Error;

	fn json<T: DeserializeOwned>(self) -> impl Future<Output = Result<T, Self::JsonError>>;
}
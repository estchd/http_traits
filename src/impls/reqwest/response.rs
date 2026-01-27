use bytes::Bytes;
use serde::de::DeserializeOwned;
use crate::impls::reqwest::status_code::ReqwestStatusCode;

pub type ReqwestResponse = reqwest::Response;

impl crate::response::Response for ReqwestResponse {
	type StatusCode = ReqwestStatusCode;

	fn status(&self) -> Self::StatusCode {
		(self as &reqwest::Response).status()
	}

	type JsonError = reqwest::Error;

	async fn json<T: DeserializeOwned>(self) -> Result<T, Self::JsonError> {
		reqwest::Response::json(self).await
	}

	type ByteError = reqwest::Error;

	async fn bytes(self) -> Result<Bytes, Self::ByteError> {
		reqwest::Response::bytes(self).await
	}
}
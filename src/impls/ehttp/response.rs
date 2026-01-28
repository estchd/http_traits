use bytes::Bytes;
use serde::de::DeserializeOwned;
use crate::impls::ehttp::error::EHTTPError;
use crate::impls::ehttp::status_code::EHTTPStatusCode;
use crate::response::Response;
use crate::status_code::StatusCode;

pub type EHTTPResponse = ehttp::Response;

impl Response for EHTTPResponse {
	type StatusCode = EHTTPStatusCode;

	fn status(&self) -> Self::StatusCode {
		Self::StatusCode::from_u16(self.status).unwrap()
	}

	type JsonError = serde_json::Error;

	async fn json<T: DeserializeOwned>(self) -> Result<T, Self::JsonError> {
		ehttp::Response::json(&self)
	}

	type ByteError = EHTTPError;

	async fn bytes(self) -> Result<Bytes, Self::ByteError> {
		Ok(Bytes::from(self.bytes))
	}

	fn content_type(&self) -> Option<String> {
		self.headers.get("content-type").map(&str::to_owned)
	}

	fn content_disposition(&self) -> Option<String> {
		self.headers.get("content-disposition").map(&str::to_owned)
	}
}
use std::error::Error;
use std::time::Duration;
use serde::Serialize;
use crate::request::Request;
use crate::response::Response;
use crate::version::Version;

pub trait RequestBuilder: Sized {
	type Request: Request;

	type BuildError: Error;

	fn build(self) -> Result<Self::Request, Self::BuildError>;

	type Response: Response;

	type SendError: Error;

	fn send(self) -> impl Future<Output = Result<Self::Response, Self::SendError>>;

	fn with_basic_auth(self, user: &str, password: Option<&str>) -> Self;

	fn with_bearer_auth(self, token: &str) -> Self;

	fn with_query<T: Serialize>(self, key: &str, value: &T) -> Self;

	fn with_header(self, key: &str, value: &str) -> Self;

	fn with_json<T: Serialize>(self, value: &T) -> Self;

	type Version: Version;

	fn with_version(self, version: Self::Version) -> Self;

	fn with_timeout(self, timeout: Duration) -> Self;

	fn try_clone(&self) -> Option<Self>;
}
use std::time::Duration;
use serde::Serialize;
use crate::impls::reqwest::request::ReqwestRequest;
use crate::impls::reqwest::response::ReqwestResponse;
use crate::impls::reqwest::version::ReqwestVersion;

pub type ReqwestRequestBuilder = reqwest::RequestBuilder;

impl crate::request_builder::RequestBuilder for ReqwestRequestBuilder {
	type Request = ReqwestRequest;
	type BuildError = reqwest::Error;

	fn build(self) -> Result<Self::Request, Self::BuildError> {
		reqwest::RequestBuilder::build(self)
	}

	type Response = ReqwestResponse;
	type SendError = reqwest::Error;

	fn send(self) -> impl Future<Output=Result<Self::Response, Self::SendError>> {
		reqwest::RequestBuilder::send(self)
	}

	fn with_basic_auth(self, user: &str, password: Option<&str>) -> Self {
		reqwest::RequestBuilder::basic_auth(self, user, password)
	}

	fn with_bearer_auth(self, token: &str) -> Self {
		reqwest::RequestBuilder::bearer_auth(self, token)
	}

	fn with_query<T: Serialize>(self, key: &str, value: &T) -> Self {
		reqwest::RequestBuilder::query(self, &[(key, value)])
	}

	fn with_header(self, key: &str, value: &str) -> Self {
		reqwest::RequestBuilder::header(self, key, value)
	}

	fn with_json<T: Serialize>(self, value: &T) -> Self {
		reqwest::RequestBuilder::json(self, value)
	}

	type Version = ReqwestVersion;

	fn with_version(self, version: Self::Version) -> Self {
		reqwest::RequestBuilder::version(self, version)
	}

	fn with_timeout(self, timeout: Duration) -> Self {
		reqwest::RequestBuilder::timeout(self, timeout)
	}

	fn try_clone(&self) -> Option<Self> {
		reqwest::RequestBuilder::try_clone(self)
	}
}
use std::time::Duration;
use ehttp::Headers;
use http_auth_basic::Credentials;
use serde::Serialize;
use url::Url;
use crate::client::Client;
use crate::impls::ehttp::client::EHTTPClient;
use crate::impls::ehttp::error::EHTTPError;
use crate::impls::ehttp::form::EHTTPForm;
use crate::impls::ehttp::method::EHTTPMethod;
use crate::impls::ehttp::request::EHTTPRequest;
use crate::impls::ehttp::request_builder::EHTTPRequestBuilderData::{Json, Multipart};
use crate::impls::ehttp::response::EHTTPResponse;
use crate::impls::ehttp::version::EHTTPVersion;
use crate::request_builder::RequestBuilder;

#[derive(Debug, Clone)]
enum EHTTPRequestBuilderData {
	None,
	Json(String),
	Multipart(String, Vec<u8>)
}

#[derive(Debug, Clone)]
pub struct EHTTPRequestBuilder {
	url: Url,
	headers: Headers,
	data: EHTTPRequestBuilderData,
	timeout: Option<Duration>,
	method: EHTTPMethod,
}

impl EHTTPRequestBuilder {
	pub fn new(url: Url, method: EHTTPMethod) -> Self {
		Self {
			url,
			headers: Headers::default(),
			data: EHTTPRequestBuilderData::None,
			timeout: Some(ehttp::Request::DEFAULT_TIMEOUT),
			method,
		}
	}
}

impl RequestBuilder for EHTTPRequestBuilder {
	type Request = EHTTPRequest;
	type BuildError = EHTTPError;

	fn build(self) -> Result<Self::Request, Self::BuildError> {
		let mut request = ehttp::Request::get(&self.url);

		let mut headers = self.headers;

		let data = match self.data {
			EHTTPRequestBuilderData::None => vec![],
			Json(json_data) => {
				headers.insert("Accept", "*/*");
				headers.insert("Content-Type", "application/json");

				json_data.into_bytes()
			},
			Multipart(content_type, multipart_data) => {
				headers.insert("Accept", "*/*");
				headers.insert("Content-Type", content_type);

				multipart_data
			}
		};

		request.body = data;
		request.headers = headers;
		request.method = self.method.method;
		request.timeout = self.timeout;

		Ok(request)
	}

	type Response = EHTTPResponse;
	type SendError = EHTTPError;

	async fn send(self) -> Result<Self::Response, Self::SendError> {
		let client = EHTTPClient {};

		let request = self.build()?;

		client.execute(request).await
	}

	fn with_basic_auth(self, user: &str, password: Option<&str>) -> Self {
		let header_value = Credentials::new(user, password.unwrap_or(""))
			.as_http_header();

		self.with_header("Authorization", &header_value)
	}

	fn with_bearer_auth(self, token: &str) -> Self {
		let header_value = format!("Bearer {}", token);

		self.with_header("Authorization", &header_value)
	}

	fn with_query<T: Serialize>(mut self, key: &str, value: &T) -> Self {
		let value = serde_urlencoded::to_string(value).unwrap();

		self.url.query_pairs_mut().append_pair(key, &value);

		self
	}

	fn with_header(mut self, key: &str, value: &str) -> Self {
		self.headers.insert(key, value);

		self
	}

	fn with_json<T: Serialize>(mut self, value: &T) -> Self {
		let value = serde_json::to_string(value).unwrap();

		self.data = Json(value);

		self
	}

	type Version = EHTTPVersion;

	fn with_version(self, _version: Self::Version) -> Self {
		unimplemented!("EHTTP does not support HTTP Versions")
	}

	fn with_timeout(mut self, timeout: Duration) -> Self {
		self.timeout = Some(timeout);

		self
	}

	type Form = EHTTPForm;

	fn with_multipart(mut self, form: Self::Form) -> Self {
		let (content_type, data) = form.finish();

		self.data = Multipart(content_type, data);

		self
	}

	fn try_clone(&self) -> Option<Self> {
		Some(self.clone())
	}
}
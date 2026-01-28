use url::Url;
use crate::client::Client;
use crate::impls::ehttp::error::EHTTPError;
use crate::impls::ehttp::method::EHTTPMethod;
use crate::impls::ehttp::request::EHTTPRequest;
use crate::impls::ehttp::request_builder::EHTTPRequestBuilder;
use crate::impls::ehttp::response::EHTTPResponse;

#[derive(Debug, Copy, Clone)]
pub struct EHTTPClient;

impl Client for EHTTPClient {
	type Request = EHTTPRequest;
	type Response = EHTTPResponse;
	type Error = EHTTPError;

	async fn execute(&self, request: Self::Request) -> Result<Self::Response, Self::Error> {
		Ok(ehttp::fetch_async(request).await?)
	}

	type Method = EHTTPMethod;
	type RequestBuilder = EHTTPRequestBuilder;

	fn request(&self, method: Self::Method, url: &Url) -> Self::RequestBuilder {
		EHTTPRequestBuilder::new(url.to_owned(), method)
	}
}
use std::error::Error;
use url::Url;
use crate::method::Method;
use crate::request::Request;
use crate::request_builder::RequestBuilder;
use crate::response::Response;

pub trait Client {
	type Request: Request;

	type Response: Response;

	type Error: Error;

	fn execute(&self, request: Self::Request) -> impl Future<Output = Result<Self::Response, Self::Error>>;

	type Method: Method;

	type RequestBuilder: RequestBuilder;

	fn request(&self, method: Self::Method, url: &Url) -> Self::RequestBuilder;

	fn get(&self, url: &Url) -> Self::RequestBuilder {
		self.request(Self::Method::get(), url)
	}

	fn head(&self, url: &Url) -> Self::RequestBuilder {
		self.request(Self::Method::head(), url)
	}

	fn patch(&self, url: &Url) -> Self::RequestBuilder {
		self.request(Self::Method::patch(), url)
	}

	fn post(&self, url: &Url) -> Self::RequestBuilder {
		self.request(Self::Method::post(), url)
	}

	fn put(&self, url: &Url) -> Self::RequestBuilder {
		self.request(Self::Method::put(), url)
	}
}
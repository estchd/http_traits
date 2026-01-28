use std::time::Duration;
use url::Url;
use crate::impls::ehttp::method::EHTTPMethod;
use crate::method::Method;
use crate::request::Request;

pub type EHTTPRequest = ehttp::Request;

impl Request for EHTTPRequest {
	type Method = EHTTPMethod;

	fn new(method: Self::Method, url: Url) -> Self {
		let mut request = ehttp::Request::get(url.as_str());

		request.method = method.method;

		request
	}

	fn timeout(&self) -> Option<Duration> {
		self.timeout
	}

	fn set_timeout(&mut self, timeout: Option<Duration>) {
		self.timeout = timeout;
	}

	fn method(&self) -> Self::Method {
		<Self::Method as Method>::custom(&self.method).unwrap()
	}

	fn set_method(&mut self, method: Self::Method) {
		self.method = method.method
	}

	fn try_clone(&self) -> Option<Self> {
		Some(self.clone())
	}
}
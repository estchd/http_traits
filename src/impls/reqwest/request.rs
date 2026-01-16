use std::time::Duration;
use url::Url;
use crate::impls::reqwest::method::ReqwestMethod;

pub type ReqwestRequest = reqwest::Request;

impl crate::request::Request for ReqwestRequest {
	type Method = ReqwestMethod;

	fn new(method: Self::Method, url: Url) -> Self {
		reqwest::Request::new(method, url)
	}

	fn timeout(&self) -> Option<Duration> {
		reqwest::Request::timeout(self).cloned()
	}

	fn set_timeout(&mut self, timeout: Option<Duration>) {
		*reqwest::Request::timeout_mut(self) = timeout;
	}

	fn method(&self) -> Self::Method {
		reqwest::Request::method(self).clone()
	}

	fn set_method(&mut self, method: Self::Method) {
		*reqwest::Request::method_mut(self) = method;
	}

	fn try_clone(&self) -> Option<Self> {
		reqwest::Request::try_clone(self)
	}
}
use std::str::FromStr;

pub type ReqwestMethod = reqwest::Method;

impl crate::method::Method for ReqwestMethod {
	type CustomError = http::method::InvalidMethod;

	fn custom(method: &str) -> Result<Self, Self::CustomError> {
		reqwest::Method::from_str(method)
	}

	fn connect() -> Self {
		reqwest::Method::CONNECT
	}

	fn delete() -> Self {
		reqwest::Method::DELETE
	}

	fn get() -> Self {
		reqwest::Method::GET
	}

	fn head() -> Self {
		reqwest::Method::HEAD
	}

	fn options() -> Self {
		reqwest::Method::OPTIONS
	}

	fn patch() -> Self {
		reqwest::Method::PATCH
	}

	fn post() -> Self {
		reqwest::Method::POST
	}

	fn put() -> Self {
		reqwest::Method::PUT
	}

	fn trace() -> Self {
		reqwest::Method::TRACE
	}
}
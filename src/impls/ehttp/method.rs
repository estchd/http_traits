use crate::impls::ehttp::error::EHTTPError;
use crate::method::Method;

#[derive(Debug, Clone)]
pub struct EHTTPMethod {
	pub(crate) method: String
}

impl Method for EHTTPMethod {
	type CustomError = EHTTPError;

	fn custom(method: &str) -> Result<Self, Self::CustomError> {
		Ok(Self{
			method: method.to_owned()
		})
	}

	fn connect() -> Self {
		Self::custom("CONNECT").unwrap()
	}

	fn delete() -> Self {
		Self::custom("DELETE").unwrap()
	}

	fn get() -> Self {
		Self::custom("GET").unwrap()
	}

	fn head() -> Self {
		Self::custom("HEAD").unwrap()
	}

	fn options() -> Self {
		Self::custom("OPTIONS").unwrap()
	}

	fn patch() -> Self {
		Self::custom("PATCH").unwrap()
	}

	fn post() -> Self {
		Self::custom("POST").unwrap()
	}

	fn put() -> Self {
		Self::custom("PUT").unwrap()
	}

	fn trace() -> Self {
		Self::custom("TRACE").unwrap()
	}
}
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum EHTTPError {
	#[error("HTTP Client Error: {0}")]
	EHTTP(String)
}

impl From<String> for EHTTPError {
	fn from(value: String) -> Self {
		Self::EHTTP(value)
	}
}
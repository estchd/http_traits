use crate::impls::ehttp::error::EHTTPError;
use crate::status_code::StatusCode;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct EHTTPStatusCode {
	code: u16,
}

impl StatusCode for EHTTPStatusCode {
	type FromU16Error = EHTTPError;

	fn from_u16(code: u16) -> Result<Self, Self::FromU16Error> {
		Ok(Self {
			code,
		})
	}

	fn as_u16(&self) -> u16 {
		self.code
	}
}
pub type ReqwestStatusCode = reqwest::StatusCode;

impl crate::status_code::StatusCode for ReqwestStatusCode {
	type FromU16Error = http::status::InvalidStatusCode;

	fn from_u16(code: u16) -> Result<Self, Self::FromU16Error> {
		reqwest::StatusCode::from_u16(code)
	}

	fn as_u16(&self) -> u16 {
		reqwest::StatusCode::as_u16(self)
	}
}
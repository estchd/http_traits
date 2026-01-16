use std::error::Error;

pub trait StatusCode: Sized {
	type FromU16Error: Error;

	fn from_u16(code: u16) -> Result<Self, Self::FromU16Error>;

	fn as_string(&self) -> String {
		format!("{}", self.as_u16())
	}

	fn as_u16(&self) -> u16;

	fn is_client_error(&self) -> bool {
		(400..=499).contains(&self.as_u16())
	}

	fn is_informational(&self) -> bool {
		(100..=199).contains(&self.as_u16())
	}

	fn is_redirection(&self) -> bool {
		(300..=399).contains(&self.as_u16())
	}

	fn is_server_error(&self) -> bool {
		(500..=599).contains(&self.as_u16())
	}

	fn is_success(&self) -> bool {
		(200..=299).contains(&self.as_u16())
	}
}
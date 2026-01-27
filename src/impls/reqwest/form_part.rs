use std::io;
use crate::form_part::FormPart;

pub type ReqwestFormPart = reqwest::multipart::Part;

impl FormPart for ReqwestFormPart {
	fn text(value: &str) -> Self {
		reqwest::multipart::Part::text(value.to_string())
	}

	fn bytes(value: &[u8]) -> Self {
		reqwest::multipart::Part::bytes(value.to_vec())
	}

	type FileError = io::Error;

	async fn file(path: &str) -> Result<Self, Self::FileError> {
		reqwest::multipart::Part::file(path.to_owned()).await
	}

	fn with_file_name(self, file_name: &str) -> Self {
		reqwest::multipart::Part::file_name(self, file_name.to_owned())
	}
}
use std::io;
use crate::form::Form;
use crate::impls::reqwest::form_part::ReqwestFormPart;

pub type ReqwestForm = reqwest::multipart::Form;

impl Form for ReqwestForm {
	type Part = ReqwestFormPart;

	fn with_part(self, name: &str, part: Self::Part) -> Self {
		reqwest::multipart::Form::part(self, name.to_owned(), part)
	}

	fn with_text(self, name: &str, value: &str) -> Self {
		reqwest::multipart::Form::text(self, name.to_owned(), value.to_owned())
	}

	type FileError = io::Error;

	async fn with_file(self, name: &str, path: &str) -> Result<Self, Self::FileError> {
		reqwest::multipart::Form::file(self, name.to_owned(), path.to_owned()).await
	}
}
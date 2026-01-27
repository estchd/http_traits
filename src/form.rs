use std::error::Error;
use crate::form_part::FormPart;

pub trait Form: Default {
	type Part: FormPart<FileError = Self::FileError>;

	fn boundary(&self) -> String;

	fn with_part(self, name: &str, part: Self::Part) -> Self;

	fn with_text(self, name: &str, value: &str) -> Self {
		self.with_part(name, Self::Part::text(value))
	}

	type FileError: Error;

	async fn with_file(self, name: &str, path: &str) -> Result<Self, Self::FileError> {
		let part = Self::Part::file(path).await?;
		Ok(self.with_part(name, part))
	}
}
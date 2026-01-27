use std::error::Error;
use crate::form::Form;

pub trait FormPart: Sized {
	fn text(value: &str) -> Self;

	fn bytes(value: &[u8]) -> Self;

	type FileError: Error;

	fn file(path: &str) -> impl Future<Output=Result<Self, Self::FileError>>;

	fn with_file_name(self, file_name: &str) -> Self;
}
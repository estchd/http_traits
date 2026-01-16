use std::error::Error;

pub trait Method: Sized {
	type CustomError: Error;

	fn custom(method: &str) -> Result<Self, Self::CustomError>;

	fn connect() -> Self;

	fn delete() -> Self;

	fn get() -> Self;

	fn head() -> Self;

	fn options() -> Self;

	fn patch() -> Self;

	fn post() -> Self;

	fn put() -> Self;

	fn trace() -> Self;
}
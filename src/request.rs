use std::time::Duration;
use url::Url;
use crate::method::Method;

pub trait Request: Sized {
	type Method: Method;

	fn new(method: Self::Method, url: Url) -> Self;

	fn timeout(&self) -> Option<Duration>;

	fn set_timeout(&mut self, timeout: Option<Duration>);

	fn method(&self) -> Self::Method;

	fn set_method(&mut self, method: Self::Method);

	fn try_clone(&self) -> Option<Self>;
}
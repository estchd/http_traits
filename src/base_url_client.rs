use url::{ParseError, Url};
use crate::client::Client;

pub trait BaseUrlClient: Client {
	fn base_url(&self) -> &Url;

	fn build_full_url(&self, endpoint: &str) -> Result<Url, ParseError> {
		self.base_url().join(endpoint)
	}
}
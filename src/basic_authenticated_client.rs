use url::Url;
use crate::client::Client;
use crate::method::Method;
use crate::request_builder::RequestBuilder;

pub trait BasicAuthenticatedClient: Client {
	fn username(&self) -> &str;

	fn password(&self) -> Option<&str>;

	fn basic_authenticated_request(&self, method: Self::Method, url: &Url) -> Self::RequestBuilder {
		self.request(method, url)
			.with_basic_auth(self.username(), self.password())
	}

	fn basic_authenticated_get(&self, url: &Url) -> Self::RequestBuilder {
		self.request(Self::Method::get(), url)
	}

	fn basic_authenticated_head(&self, url: &Url) -> Self::RequestBuilder {
		self.request(Self::Method::head(), url)
	}

	fn basic_authenticated_patch(&self, url: &Url) -> Self::RequestBuilder {
		self.request(Self::Method::patch(), url)
	}

	fn basic_authenticated_post(&self, url: &Url) -> Self::RequestBuilder {
		self.request(Self::Method::post(), url)
	}

	fn basic_authenticated_put(&self, url: &Url) -> Self::RequestBuilder {
		self.request(Self::Method::put(), url)
	}
}
use url::Url;
use crate::client::Client;
use crate::method::Method;
use crate::request_builder::RequestBuilder;

pub trait BearerAuthenticatedClient: Client {
	fn authentication_token(&self) -> &str;

	fn bearer_authenticated_request(&self, method: Self::Method, url: &Url) -> Self::RequestBuilder {
		self.request(method, url)
			.with_bearer_auth(self.authentication_token())
	}

	fn bearer_authenticated_get(&self, url: &Url) -> Self::RequestBuilder {
		self.bearer_authenticated_request(Self::Method::get(), url)
	}

	fn bearer_authenticated_head(&self, url: &Url) -> Self::RequestBuilder {
		self.bearer_authenticated_request(Self::Method::head(), url)
	}

	fn bearer_authenticated_patch(&self, url: &Url) -> Self::RequestBuilder {
		self.bearer_authenticated_request(Self::Method::patch(), url)
	}

	fn bearer_authenticated_post(&self, url: &Url) -> Self::RequestBuilder {
		self.bearer_authenticated_request(Self::Method::post(), url)
	}

	fn bearer_authenticated_put(&self, url: &Url) -> Self::RequestBuilder {
		self.bearer_authenticated_request(Self::Method::put(), url)
	}
}
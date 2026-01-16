use reqwest::Url;

pub type ReqwestClient = reqwest::Client;

impl crate::client::Client for ReqwestClient {
	type Request = reqwest::Request;
	type Response = reqwest::Response;
	type Error = reqwest::Error;

	async fn execute(&self, request: Self::Request) -> Result<Self::Response, Self::Error> {
		reqwest::Client::execute(self, request).await
	}

	type Method = reqwest::Method;
	type RequestBuilder = reqwest::RequestBuilder;

	fn request(&self, method: Self::Method, url: &Url) -> Self::RequestBuilder {
		reqwest::Client::request(self, method, url.clone())
	}
}
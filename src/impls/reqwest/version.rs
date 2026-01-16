pub type ReqwestVersion = reqwest::Version;

impl crate::version::Version for ReqwestVersion {
	fn http_09() -> Self {
		reqwest::Version::HTTP_09
	}

	fn http_2() -> Self {
		reqwest::Version::HTTP_2
	}

	fn http_3() -> Self {
		reqwest::Version::HTTP_3
	}

	fn http_10() -> Self {
		reqwest::Version::HTTP_10
	}

	fn http_11() -> Self {
		reqwest::Version::HTTP_11
	}
}
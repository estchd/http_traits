use crate::version::Version;

pub struct EHTTPVersion {

}

impl Version for EHTTPVersion {
	fn http_09() -> Self {
		unimplemented!("EHTTP does not support HTTP Versions")
	}

	fn http_2() -> Self {
		unimplemented!("EHTTP does not support HTTP Versions")
	}

	fn http_3() -> Self {
		unimplemented!("EHTTP does not support HTTP Versions")
	}

	fn http_10() -> Self {
		unimplemented!("EHTTP does not support HTTP Versions")
	}

	fn http_11() -> Self {
		unimplemented!("EHTTP does not support HTTP Versions")
	}
}
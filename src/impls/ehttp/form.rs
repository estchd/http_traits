use std::str::FromStr;
use ehttp::multipart::MultipartBuilder;
use mime::Mime;
use crate::form::Form;
use crate::impls::ehttp::form_part::{EHTTPFormPart, EHTTPFormPartData};

pub type EHTTPForm = MultipartBuilder;

impl Form for EHTTPForm {
	type Part = EHTTPFormPart;

	fn with_part(mut self, name: &str, part: Self::Part) -> Self {
		match part.data {
			EHTTPFormPartData::Text(text) => {
				self = self.add_text(name, &text);
			}
			EHTTPFormPartData::Bytes(mut bytes) => {
				let content_type = Mime::from_str("application/octet-stream").unwrap();
				let mut data = bytes.as_slice();
				self = self.add_stream::<&[u8]>(
					&mut data,
					name,
					part.file_name.as_deref(),
					Some(content_type),
				).unwrap();
			}
			EHTTPFormPartData::File(content_type, data) => {
				let content_type = Mime::from_str(&content_type).unwrap();
				let mut data = data.as_slice();
				self = self.add_stream::<&[u8]>(
					&mut data,
					name,
					part.file_name.as_deref(),
					Some(content_type),
				).unwrap();
			}
		}

		self
	}

	type FileError = std::io::Error;
}
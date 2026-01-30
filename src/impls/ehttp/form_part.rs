use std::io::Read;
use crate::form_part::FormPart;
use crate::impls::ehttp::form_part::EHTTPFormPartData::{Bytes, File, Text};

#[derive(Debug, Clone)]
pub(super) enum EHTTPFormPartData {
	Text(String),
	Bytes(Vec<u8>),
	File(String, Vec<u8>)
}

#[derive(Debug, Clone)]
pub struct EHTTPFormPart {
	pub(super) file_name: Option<String>,
	pub(super) data: EHTTPFormPartData
}

impl FormPart for EHTTPFormPart {
	fn text(value: &str) -> Self {
		Self {
			file_name: None,
			data: Text(value.to_owned()),
		}
	}

	fn bytes(value: &[u8]) -> Self {
		Self {
			file_name: None,
			data: Bytes(value.to_owned())
		}
	}

	type FileError = std::io::Error;

	fn file_bytes(mime: String, bytes: &[u8]) -> Self {
		Self {
			file_name: None,
			data: File(mime, bytes.to_owned())
		}
	}

	async fn file(path: &str) -> Result<Self, Self::FileError> {
		let mime = mime_guess::from_path(path).first_or_octet_stream()
			.to_string();

		let mut file = std::fs::File::open(path)?;

		let mut data = vec![];

		file.read_to_end(&mut data)?;


		Ok(Self {
			file_name: None,
			data: File(mime, data),
		})
	}

	fn with_file_name(mut self, file_name: &str) -> Self {
		self.file_name = Some(file_name.to_owned());

		self
	}
}
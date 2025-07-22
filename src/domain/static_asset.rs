use bytes::Bytes;
use http::HeaderMap;
use http::Response;
use http::header;
use rust_embed::EmbeddedFile;

use crate::error::ApplicationError;

/// Represents a static asset that can be served by a http server.
#[derive(Debug, Clone)]
pub(crate) struct StaticAsset(pub(crate) Response<Bytes>);

impl StaticAsset {
    /// Convert from a `rust_embed::EmbeddedFile` to a [`StaticAsset`]. `path`
    /// is the path to the asset (this is used to guess the mime type).
    pub fn from_rust_embed<P: AsRef<std::path::Path>>(
        embedded_file: EmbeddedFile,
        path: P,
    ) -> Result<Self, ApplicationError> {
        let mime = mime_guess::from_path(&path).first_or_octet_stream();

        let mut headers = HeaderMap::new();
        headers.append(
            header::CONTENT_TYPE,
            mime.as_ref().parse().map_err(|_err| {
                ApplicationError::ServerError(format!(
                    "unable to generate valid header from mime type: {mime}"
                ))
            })?,
        );

        let mut response = Response::builder()
            .body(Bytes::from(embedded_file.data.into_owned()))
            .map_err(|_err| {
                ApplicationError::ServerError(format!(
                    "unable to convert `{}` to bytes",
                    path.as_ref().to_string_lossy()
                ))
            })?;

        *response.headers_mut() = headers;

        Ok(Self(response))
    }
}

#[cfg(test)]
impl StaticAsset {
    /// Return the data bytes associated with the asset.
    pub fn data(&self) -> Vec<u8> {
        self.0.body().to_vec()
    }
}

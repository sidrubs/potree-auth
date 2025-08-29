use std::path::PathBuf;

use bytes::Bytes;
use http::HeaderMap;
use http::HeaderValue;
use http::Response;
use http::header;
use rust_embed::EmbeddedFile;

use crate::common::domain::utils::last_modified::http_date_from_unix_time;

/// Represents a static asset that can be served by a http server.
#[derive(Debug, Clone)]
pub struct StaticAsset(pub Response<Bytes>);

impl StaticAsset {
    /// Convert from a `rust_embed::EmbeddedFile` to a [`StaticAsset`]. `path`
    /// is the path to the asset (this is used to guess the mime type).
    pub fn from_rust_embed<P: AsRef<std::path::Path>>(
        embedded_file: EmbeddedFile,
        path: P,
    ) -> Result<Self, StaticAssetError> {
        let mime = mime_guess::from_path(&path).first_or_octet_stream();

        let mut headers = HeaderMap::new();
        headers.append(
            header::CONTENT_TYPE,
            mime.as_ref()
                .parse()
                .map_err(|_e| StaticAssetError::InvalidContentTypeHeader {
                    mime_type: mime.to_string(),
                })?,
        );

        // Add Last-Modified header to help with caching
        if let Some(last_modified) = embedded_file.metadata.last_modified() {
            let http_date = http_date_from_unix_time(last_modified);

            headers.append(
                header::LAST_MODIFIED,
                HeaderValue::from_str(&http_date.to_string()).map_err(|_e| {
                    StaticAssetError::InvalidLastModifiedHeader {
                        http_date: http_date.to_string(),
                    }
                })?,
            );
        }

        let mut response = Response::builder()
            .body(Bytes::from(embedded_file.data.into_owned()))
            .map_err(|_e| StaticAssetError::BytesConversion {
                path: path.as_ref().to_path_buf(),
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

#[derive(Debug, Clone, thiserror::Error)]
pub enum StaticAssetError {
    #[error("unable to create content-type header from {mime_type}")]
    InvalidContentTypeHeader { mime_type: String },

    #[error("unable to create last-modified header from {http_date}")]
    InvalidLastModifiedHeader { http_date: String },

    #[error("unable to convert {path} to bytes")]
    BytesConversion { path: PathBuf },
}

use std::borrow::Cow;

use mime::Mime;
use rust_embed::EmbeddedFile;

/// Represents a static asset that can be served by a http server.
#[derive(Debug, Clone)]
pub(crate) struct StaticAsset {
    /// The binary data of the asset.
    pub data: Cow<'static, [u8]>,

    /// The asset mime type.
    pub mime: Mime,
}

impl StaticAsset {
    /// Convert from a `rust_embed::EmbeddedFile` to a [`StaticAsset`]. `path`
    /// is the path to the asset (this is used to guess the mime type).
    pub fn from_rust_embed<P: AsRef<std::path::Path>>(
        embedded_file: EmbeddedFile,
        path: P,
    ) -> Self {
        let mime = mime_guess::from_path(path).first_or_octet_stream();

        Self {
            data: embedded_file.data,
            mime,
        }
    }
}

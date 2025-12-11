use std::io::Write;

use anyhow::Result;
use flate2::{Compression, write::GzEncoder};
use turbo_rcstr::RcStr;

/// Compresses an asset's content with default-level (level 6) gzip, returning the number of bytes
/// present in the compressed output
// #[turbo_tasks::function]
pub fn compressed_size_bytes(content: RcStr) -> Result<u32> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(content.as_ref())?;
    let compressed = encoder.finish()?;

    Ok(compressed.len() as u32)
}

#![cfg_attr(windows, feature(abi_vectorcall))]
#[cfg(not(test))]
use ext_php_rs::{binary::Binary, prelude::*};

use std::io::{self, Read, Write};

use lz4_flex::{block::DecompressError, compress_prepend_size, decompress_size_prepended, frame::{BlockMode, BlockSize, FrameDecoder, FrameEncoder, FrameInfo}};

#[cfg(not(test))]
#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .function(wrap_function!(lz4_compress))
        .function(wrap_function!(lz4_uncompress))
        .function(wrap_function!(lz4frame_compress))
        .function(wrap_function!(lz4frame_uncompress))
}

/// Compress data to LZ4 block
/// @param string data to be compressed
/// @return string LZ4 block
#[cfg(not(test))]
#[php_function]
#[php(name = "lz4_compress")]
fn lz4_compress(data: Binary<u8>) -> Binary<u8> {
    _lz4_compress(data.into()).into()
}

fn _lz4_compress(data: Vec<u8>) -> Vec<u8> {
    compress_prepend_size(&data)
}

/// Decompress data from LZ4 block
/// @param string compressed LZ4 block
/// @return string decompressed data
#[cfg(not(test))]
#[php_function]
#[php(name = "lz4_uncompress")]
fn lz4_uncompress(compressed: Binary<u8>) -> PhpResult<Binary<u8>> {
    let output = _lz4_uncompress(compressed.into()).map_err(|err| err.to_string())?;

    Ok(output.into())
}

fn _lz4_uncompress(compressed: Vec<u8>) -> Result<Vec<u8>, DecompressError> {
    let output = decompress_size_prepended(&compressed)?;

    Ok(output)
}

/// Compress data to LZ4 frame
/// @param string data to be compressed
/// @param int|null content_size if set, total uncompressed data size in bytes, will be unspecified in the LZ4 frame otherwise
/// @param int block_size 4: max 64 KB, 5: max 256 KB, 6: max 1 MB, 7: max 4 MB, 8: max 8 MB, any other value: auto
/// @param bool blocks_linked if true, consecutive blocks can reference data from previous blocks (prevents parallelization, might be worth it for small blocks)
/// @param bool block_checksums if true, includes a checksum for each data block
/// @param bool content_checksums if true, includes a checksum for the whole content of the frame (the header uses a checksum anyway)
/// @param bool legacy_frame if true, uses legacy frame format (not recommended)
/// @return string LZ4 frame
#[cfg(not(test))]
#[php_function]
#[php(
    name = "lz4frame_compress", 
    optional = "content_size",
    defaults(content_size = None, max_block_size = 4, blocks_linked = false, block_checksums = false, content_checksum = false, legacy_frame = false),
)]
fn lz4frame_compress(data: Binary<u8>, content_size: Option<u64>, max_block_size: u8, blocks_linked: bool, block_checksums: bool, content_checksum: bool, legacy_frame: bool) -> PhpResult<Binary<u8>> {
    let res = _lz4frame_compress(data.into(), content_size, max_block_size, blocks_linked, block_checksums, content_checksum, legacy_frame);
    res.map(|v| Ok(v.into())).map_err(|err| err.to_string())?
}

fn _lz4frame_compress(data: Vec<u8>, content_size: Option<u64>, max_block_size: u8, blocks_linked: bool, block_checksums: bool, content_checksum: bool, legacy_frame: bool) -> Result<Vec<u8>, io::Error> {
    let frame_info = FrameInfo::new()
        .content_size(content_size)
        .block_size(match max_block_size {
            0 => BlockSize::Auto,
            4 => BlockSize::Max64KB,
            5 => BlockSize::Max256KB,
            6 => BlockSize::Max1MB,
            7 => BlockSize::Max4MB,
            8 => BlockSize::Max8MB,
            _ => BlockSize::Auto
        })
        .block_mode(if blocks_linked { BlockMode::Linked } else { BlockMode::Independent })
        .block_checksums(block_checksums)
        .content_checksum(content_checksum)
        .legacy_frame(legacy_frame);
    let output = vec![];
    let mut encoder = FrameEncoder::with_frame_info(frame_info, output);

    encoder.write(&data[..])?;
    encoder.flush()?;
    let output = encoder.finish()?;

    Ok(output)
}

/// Decompress data from LZ4 frame
/// @param string compressed LZ4 frame
/// @return string decompressed data
#[cfg(not(test))]
#[php_function]
#[php(name = "lz4frame_uncompress")]
fn lz4frame_uncompress(compressed: Binary<u8>) -> PhpResult<Binary<u8>> {
    let res = _lz4frame_uncompress(compressed.into());
    res.map(|v| Ok(v.into())).map_err(|err| err.to_string())?
}

fn _lz4frame_uncompress(compressed: Vec<u8>) -> Result<Vec<u8>, io::Error> {
    let mut decoder = FrameDecoder::new(&compressed[..]);
    let mut output = vec![];

    decoder.read_to_end(&mut output)?;

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lz4block() {
        let data = b"Hello, world!".to_vec();
        let compressed = _lz4_compress(data.clone());

        let original_res = _lz4_uncompress(compressed);
        assert!(original_res.is_ok());
        assert_eq!(data, original_res.unwrap());
    }

    #[test]
    fn test_lz4frame() {
        let data = b"Hello, world!".to_vec();
        let compressed_res = _lz4frame_compress(data.clone(), None, 4, false, false, false, false);
        assert!(compressed_res.is_ok());
        let compressed = compressed_res.unwrap();
        let raw: Vec<u8> = compressed.clone().into();
        // compare magic bytes
        assert_eq!(0x04, raw[0]);
        assert_eq!(0x22, raw[1]);
        assert_eq!(0x4d, raw[2]);
        assert_eq!(0x18, raw[3]);
        // compare end mark
        assert_eq!(0x00, raw[raw.len() - 1]);
        assert_eq!(0x00, raw[raw.len() - 2]);
        assert_eq!(0x00, raw[raw.len() - 3]);
        assert_eq!(0x00, raw[raw.len() - 4]);

        let uncompressed_res = _lz4frame_uncompress(compressed);
        assert!(uncompressed_res.is_ok());
        let uncompressed = uncompressed_res.unwrap();
        let original: Vec<u8> = uncompressed.into();
        assert_eq!(data, original);
    }
}

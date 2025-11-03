<?php

// Stubs for lz4frame

namespace {
    /**
     * Compress data to LZ4 block
     * @param string data to be compressed
     * @return string LZ4 block
     */
    function lz4_compress(string $data): string {}

    /**
     * Decompress data from LZ4 block
     * @param string compressed LZ4 block
     * @return string decompressed data
     */
    function lz4_uncompress(string $compressed): string {}

    /**
     * Compress data to LZ4 frame
     * @param string data to be compressed
     * @param int|null content_size if set, total uncompressed data size in bytes, will be unspecified in the LZ4 frame otherwise
     * @param int max_block_size 4: max 64 KB, 5: max 256 KB, 6: max 1 MB, 7: max 4 MB, 8: max 8 MB, any other value: auto
     * @param bool blocks_linked if true, consecutive blocks can reference data from previous blocks (prevents parallelization, might be worth it for small blocks)
     * @param bool block_checksums if true, includes a checksum for each data block
     * @param bool content_checksums if true, includes a checksum for the whole content of the frame (the header uses a checksum anyway)
     * @param bool legacy_frame if true, uses legacy frame format (not recommended)
     * @return string LZ4 frame
     */
    function lz4frame_compress(string $data, ?int $content_size = null, int $block_size = 4, bool $blocks_linked = false, bool $block_checksums = false, bool $content_checksum = false, bool $legacy_frame = false): string {}

    /**
     * Decompress data from LZ4 frame
     * @param string compressed LZ4 frame
     * @return string decompressed data
     */
    function lz4frame_uncompress(string $compressed): string {}
}

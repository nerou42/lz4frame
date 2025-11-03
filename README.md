# LZ4 extension for PHP supporting block and frame format

This extension adds support for the [LZ4 compression algorithm](https://github.com/lz4/lz4).

Unlike [kjdev/lz4](https://github.com/kjdev/php-ext-lz4), this extension...

- is based on [lz4_flex](https://github.com/pseitz/lz4_flex)
- supports the LZ4 Frame format, which adds checksums, magic bytes etc.

## Install

It is recommended to install this extension via PIE:

```shell
pie install nerou/lz4frame
```

## License

This project is licensed under the [MIT license](LICENSE.md).

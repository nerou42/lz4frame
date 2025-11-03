<?php
declare(strict_types=1);

namespace Test;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\Attributes\Depends;
use PHPUnit\Framework\TestCase;

class LZ4FrameTest extends TestCase {

  public static function dataProvider(): array {
    $str = 'Hello, world!';
    return [
      [$str, false, false],
      [$str, true, false],
      [$str, false, true],
      [$str, true, true],
      [bin2hex(random_bytes(16_000_000 / 2)), true, false]   // 16 million chars
    ];
  }

  #[DataProvider('dataProvider')]
  public function testFrameCompress(string $data, bool $contentSize, bool $contentChecksum): string {
    if($contentSize){
      $compressed = lz4frame_compress($data, mb_strlen($data, '8bit'), content_checksum: $contentChecksum);
    } else {
      $compressed = lz4frame_compress($data);
    }
    $this->assertIsString($compressed);
    $this->assertEquals(0, mb_strpos($compressed, "\x04\x22\x4d\x18"));
    $this->assertNotFalse(mb_strpos($compressed, "\x00\x00\x00\x00"));
    return $compressed;
  }

  #[DataProvider('dataProvider')]
  #[Depends('testFrameCompress')]
  public function testFrameDecompress(string $data, bool $contentSize, bool $contentChecksum): void {
    $compressed = lz4frame_compress($data, $contentSize ? mb_strlen($data, '8bit') : null, content_checksum: $contentChecksum);
    $decompressed = lz4frame_uncompress($compressed);
    $this->assertEquals($data, $decompressed);
  }
}

lazycsv
========
Vectorized, lazy-decoding, zero-copy CSV parser.

[Benchmarks](https://lazycsv.contentstech.com)

## Primary Focuses

- **Vectorized**: The parser utilizes SIMD operations, therefore is very performant.
- **Minimal hidden costs**: Every API doesn't bring any invisible overheads, and each operation only does what it needs to do.
- **Zero copy, zero allocation by default**: The parser doesn't allocate any memory during parsing and only performs allocation when dequoting each cell.
- **Lazy Decoding**: Input is not copied or unquoted until requested. This is useful when you only need to access a few cells in a large CSV file.
- **`#![no_std]` eligible**: The crate is `#![no_std]` compatible, and it can be used in systems without an allocator.

## Supported Features

`lazycsv` primarily supports a subset of [RFC 4180](https://datatracker.ietf.org/doc/html/rfc4180) with minor extensions.

### According to RFC 4180:

- No escape mechanisms other than quoting are supported.
- Padding cells with whitespace is not allowed.
- Using double quotes without quoting is not allowed.
- Quotes must always appear at the very beginning of a cell.

### Additional Restrictions:

- Only ASCII and UTF-8 encodings are supported.

### Additional Supports:

- Using LF (`\n`) instead of CRLF (`\r\n`) as the newline is permitted.
- Customizing the separator character is possible.

## Examples

```rust
use lazycsv::{Csv, CsvIterItem};

// Iterating over rows
let csv = Csv::new(b"a,b,c\n1,2,3");
for [first, second, third] in csv.into_rows() {
    println!(
        "{}, {}, {}",
        first.try_as_str()?,
        second.try_as_str()?,
        third.try_as_str()?,
    );
}

// Or if you want to avoid buffering:
let csv2 = Csv::new(b"a,b,c\n1,2,3");
for item in csv2 {
    if let CsvIterItem::Cell(cell) = item {
        println!("{}", cell.try_as_str()?);
    }
}
```

&nbsp;

--------

*lazycsv* is primarily distributed under the terms of both the [Apache License
(Version 2.0)] and the [MIT license]. See [COPYRIGHT] for details.

[MIT license]: LICENSE-MIT
[Apache License (Version 2.0)]: LICENSE-APACHE
[COPYRIGHT]: COPYRIGHT

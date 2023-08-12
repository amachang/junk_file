# junk\_file: Detect Common Junk Files

This library helps you detect commonly generated junk files like `.DS_Store` or `Thumbs.db`.

## Features

1. Detect common junk files such as `.DS_Store` and `Thumbs.db`.
2. Auto-update: Uses GitHub Actions to automatically reflect changes from the [original node junk project](https://github.com/sindresorhus/junk) within a week.

If you discover new junk files, please consider reporting to the [original junk project](https://github.com/sindresorhus/junk).

## Examples

```rust
use junk_file::*;
use std::ffi::OsString;

assert_eq!(is_junk("Thumbs.db"), true);
assert_eq!(is_junk(OsString::from(".DS_Store")), true); // also OsStr, OsString acceptable

assert_eq!(is_not_junk("filename.txt"), true);
assert_eq!(is_not_junk(OsString::from("filename.txt")), true);
```

License: MIT OR Apache-2.0


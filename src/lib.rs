use regex::Regex;
use lazy_static::lazy_static;
use std::ffi::OsStr;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

lazy_static! {
    static ref JUNK_REGEX: Regex = 
        Regex::new(JUNK_REGEX_STR).expect("Since a PCRE should be generated automatically, there should be no error here.");
}

/// The junk filename regex pattern.
pub fn regex() -> &'static Regex {
    &JUNK_REGEX
}

/// Checks if the provided filename is junk.
///
/// This function can accept various string types, including:
/// - `&str`
/// - `String`
/// - `&OsStr`
/// - `OsString`
///
/// # Examples
///
/// ```rust
/// use junk_file::is_junk;
/// use std::ffi::OsString;
///
/// assert_eq!(is_junk("Thumbs.db"), true);
/// assert_eq!(is_junk(OsString::from(".DS_Store")), true);
/// ```
pub fn is_junk(filename: impl AsRef<OsStr>) -> bool {
    let regex = regex();
    match filename.as_ref().to_str() {
        Some(filename) => regex.is_match(filename),
        None => false,
    }
}

/// Checks if the provided filename is normal not a junk file like .DS_Store/Thumbs.db.
///
/// This function can accept various string types, including:
/// - `&str`
/// - `String`
/// - `&OsStr`
/// - `OsString`
///
/// # Examples
///
/// ```rust
/// use junk_file::is_not_junk;
/// use std::ffi::OsString;
///
/// assert_eq!(is_not_junk("filename.txt"), true);
/// assert_eq!(is_not_junk(OsString::from("filename.txt")), true);
/// ```
pub fn is_not_junk(filename: impl AsRef<OsStr>) -> bool {
    !is_junk(filename)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let fixture = vec![
            ".DS_Store",
            ".AppleDouble",
            ".LSOverride",
            "Icon\r",
            "._test",
            ".Spotlight-V100",
            ".Spotlight-V100/Store-V2/C6DBF25D-81D4-4B57-907E-B4A555E72C90/0.directoryStoreFile",
            ".Trashes",
            "__MACOSX",
            "test~",
            "Thumbs.db",
            "ehthumbs.db",
            "Desktop.ini",
            "desktop.ini",
            "npm-debug.log",
            ".test.swp",
            "@eaDir",
        ];

        let not_fixture = vec![
            "test",
            "Icon",
            "Icons.woff",
            ".Spotlight-V100-unicorn",
        ];

        for filename in fixture {
            assert!(is_junk(filename), "{:?} is junk", filename);
        }

        for filename in not_fixture {
            assert!(is_not_junk(filename), "{:?} is not junk", filename);
        }
    }
}

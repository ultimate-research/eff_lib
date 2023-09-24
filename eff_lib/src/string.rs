use std::{
    convert::Infallible,
    io::{Read, Seek, Write},
    str::{self, FromStr},
};

use binrw::{BinRead, BinResult, BinWrite, Endian};

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A nul-terminated string with a 1-byte alignment.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CString(Vec<u8>);

impl CString {
    /// Constructs a new instance of [`CString`] from a byte slice.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use eff_lib::CString;
    ///
    /// let s = CString::from_bytes(b"MARIO_FINAL_BULLET\0");
    /// assert_eq!(s, "MARIO_FINAL_BULLET");
    /// ```
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self(bytes.iter().copied().take_while(|b| *b != 0u8).collect())
    }

    /// Returns the length of the contained string.
    ///
    /// This length is in bytes, not [`char`]s or graphemes. In other words,
    /// it might not be what a human considers the length of the string.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use eff_lib::CString;
    ///
    /// let s = CString::from_bytes(b"M_MarioFinalBullet\0");
    /// assert_eq!(s.len(), 18);
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the contained string has a length of zero, and `false` otherwise.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use eff_lib::CString;
    ///
    /// let s = CString::from_bytes(b"\0");
    /// assert!(s.is_empty());
    ///
    /// let s = CString::from_bytes(b"bulletA1\0");
    /// assert!(!s.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Converts the underlying buffer to a string slice if it contains valid UTF-8.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use eff_lib::CString;
    ///
    /// let s = CString::from_bytes(b"bulletA3\0");
    /// assert_eq!(s.to_str().unwrap(), "bulletA3");
    /// ```
    pub fn to_str(&self) -> Result<&str, str::Utf8Error> {
        str::from_utf8(&self.0)
    }

    /// Converts the underlying buffer to a [`String`] if it contains valid UTF-8.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use eff_lib::CString;
    ///
    /// let s = CString::from_bytes(b"bulletB1\0");
    /// assert_eq!(s.to_string().unwrap(), "bulletB1".to_string());
    /// ```
    pub fn to_string(&self) -> Result<String, str::Utf8Error> {
        self.to_str().map(|s| s.to_string())
    }
}

impl FromStr for CString {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl From<&str> for CString {
    fn from(value: &str) -> Self {
        Self(value.as_bytes().into())
    }
}

impl From<String> for CString {
    fn from(value: String) -> Self {
        Self(value.as_bytes().into())
    }
}

impl From<&String> for CString {
    fn from(value: &String) -> Self {
        Self(value.as_bytes().into())
    }
}

impl PartialEq<&str> for CString {
    fn eq(&self, other: &&str) -> bool {
        self.0 == other.as_bytes()
    }
}

impl PartialEq<String> for CString {
    fn eq(&self, other: &String) -> bool {
        self.0 == other.as_bytes()
    }
}

impl PartialEq<&String> for CString {
    fn eq(&self, other: &&String) -> bool {
        self.0 == other.as_bytes()
    }
}

impl BinRead for CString {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        _endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let mut bytes = Vec::new();

        loop {
            let b = u8::read(reader)?;

            if b == 0 {
                return Ok(Self(bytes));
            }

            bytes.push(b);
        }
    }
}

impl BinWrite for CString {
    type Args<'a> = ();

    fn write_options<R: Write + Seek>(
        &self,
        writer: &mut R,
        _endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<()> {
        if !self.0.is_empty() {
            writer.write_all(&self.0)?;
        }

        writer.write_all(&[0u8])?;

        Ok(())
    }
}

#[cfg(feature = "serde")]
impl Serialize for CString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_str().unwrap())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for CString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;

        Self::from_str(&string).map_err(serde::de::Error::custom)
    }
}



use std::{ops::{AddAssign, MulAssign}, process::Output};

use crate::error::{Error, Result};
use serde::Deserialize;

pub struct Deserializer<'de> {
    /// The string starts with the input data and characters are truncated off the beginning as data is parsed.
    input: &'de str,
}

impl<'de> Deserializer<'de> {
    /// By convecntion, `Deserializer` contructors are named like `from_xyz`.
    /// That way basic use cases ares satisfied by something like `serde_json::from_str(...)` while advanced use cases
    /// that require a deserializer can make one with `serde_json::Deserializer::from_str(....)`
    pub fn from_str(input: &'de str) -> Self {
        Deserializer { input }
    }
}

/// By convention, the public API of a Serde deserializer is one or more
/// `from_xyz` methods such as `from_str`, `from_bytes` or `from_reader`
/// depending on what Reust types the deserializer is able to consume as input.
/// 
/// This basic deserializer supports only `from_str`
pub fn from_str<'a,T>(s: &'a str)-> Result<T> where T: Deserialize<'a> {
    let mut deserializer = Deserializer::from_str(s);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(Error::TrailingCharacters)
    }
}

/// SERDE IS NOT A PARSING LIBRARY. THis impl block deifnes a few basic parsing
/// functions from scratch. More complicated formats may wish to use a dedicated pasing library to help
/// implement their Serde deserializer.
impl<'de> Deserializer<'de> {

    // Look at the first character in the input without consuming it.
    fn peek_char(&mut self) -> Result<char> {
        self.input.chars().next().ok_or(Error::Eof)
    }

    fn next_char(&mut self) -> Result<char> {
        let ch =  self.peek_char()?;
        self.input = &self.input[ch.len_utf8()..];
        Ok(ch)
    }

    fn parse_bool(&mut self) -> Result<bool> {
        if self.input.starts_with("true") {
            self.input = &self.input["true".len()..];
            Ok(true)
        } else if self.input.starts_with("false") {
            self.input = &self.input["false".len()..];
            Ok(false)
        } else {
            Err(Error::ExpectedBoolean)
        }
     }

    fn parse_unsigned<T>(&mut self) -> Result<T> where T: AddAssign<T> + MulAssign<T> +From<u8>{
        
        let mut int =match self.next_char()? {
            ch @ '0'..='9' => T::from(ch as u8 - b'0'),
            _ => {
                return Err(Error::ExpectedInteger);
            }
        };

        loop {
            match self.input.chars().next() {
                Some(ch @ '0' ..='9') => {
                    self.input = &self.input[1..];
                    int*=T::from(10);
                    int += T::from(ch as u8 - b'0');
                }
                _ => {
                    return Ok(int)
                }
            }
        }
    }

    fn parse_signed<T>(&mut self) -> Result<T> where T: Neg<Output =T> + AddAssign<T> + MulAssign<T> + From<i8> {
        unimplemented!()
    }

    fn parse_string(&mut self) -> Result<&'de str> {
        if self.next_char()? != '"' {
            return Err(Error::ExpectedString);
        }
        match self.input.find('"') {
            Some(len) => {
                let s = &self.input[..len];
                self.input = &self.input[len+1 ..];
                Ok(s)
            }
            None => Err(Error::Eof)
        }
    }
}
use std::{fs::File, io::{BufReader, Cursor, Read, Seek}, iter::Peekable};

use crate::{reader::JsonReader, value::Number};


#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    CurlyOpen,
    CurlyClose,
    Quotes,
    Colon,
    String(String),
    Number(Number),
    ArrayOpen,
    ArrayClose,
    Comma,
    Boolean(bool),
    Null
}


pub struct Jsontokensizer<T> where T: Read + Seek {
    tokens: Vec<Token>,
    iterator: Peekable<JsonReader<T>>
}

impl<T> Jsontokensizer<T> where  T: Read + Seek {
    pub fn new(reader: File) -> Jsontokensizer<File> {
        let json_reader = JsonReader::<File>::new(BufReader::new(reader));
        Jsontokensizer { tokens: vec![], iterator: json_reader.peekable() }
    }

    pub fn from_bytes<'a>(input: &'a [u8]) -> Jsontokensizer<Cursor<&'a [u8]>> {
        let json_reader = JsonReader::<Cursor<&'a [u8]>>::from_bytes(input);
        Jsontokensizer { tokens: Vec::with_capacity(input.len()), iterator: json_reader.peekable() }
    }
}

impl<T> Jsontokensizer<T> where T: Read + Seek {
    pub fn tokensize_json(&mut self) -> Result<&[Token], ()> {
        while let Some(character) = self.iterator.peek() {
            match *character {
                '"' => {
                    self.tokens.push(Token::Quotes);
                    let _ = self.iterator.next();

                    let string = self.parse_string();

                    self.tokens.push(Token::String(string));

                    self.tokens.push(Token::Quotes);
                }
                character => {
                    if character.is_ascii_whitespace() {
                        continue;
                    }
                    panic!("Unexpected character: ;{character};")
                }
            }
        }
        Ok(&self.tokens)
    }


    fn parse_string(&mut self) -> String {
        let mut string_characters = Vec::<char>::new();

        for character in self.iterator.by_ref() {
            if character == '"' {
                break;
            }

            string_characters.push(character);
        }
        String::from_iter(string_characters)
    }
}
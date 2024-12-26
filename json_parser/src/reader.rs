use std::{
    collections::VecDeque,
    io::{BufReader, Cursor, Read, Seek}, str::from_utf8,
};

/// We are going to support two different readers. One is directly from a
/// buffered reader (which is most commonly created from a file), and the
/// other is from a iterator over bytes.
///
/// For reading from a file, we need to create buffered cursor over
/// underlying file data
///
/// ```ignore
/// let file = File::create("dummy.json").unwrap();
/// let reader = BufReader::neW(file);
/// ```

/// Starting by implementing the JSON parser struct

/// A struct that handles reading input data to be parsed and
/// provides an iterator over said data character-by-character
pub struct JsonReader<T>
where
    T: Read + Seek,
{
    /// A reference to the input data, which can be anything
    /// that implements [`Read`]
    reader: BufReader<T>,

    /// A character buffer that holds queue of characters to
    /// be used by the iterator.
    ///
    /// This is necessary because UTF-8 can be 1-4 bytes long.
    /// Because of this, the reader always reads 4 bytes at a
    /// time. We then iterate over "characters", irrespective of
    /// whether they are 1 byte long, or 4
    ///
    /// A [`VecDeque`] is used instead of a normal vector
    /// because characters need to be read out from the start
    /// of the buffer.
    character_buffer: VecDeque<char>,
}

impl<T> JsonReader<T>
where
    T: Read + Seek,
{
    /// Create a new [`JsonReader`] that reads from a file
    /// 
    /// # Example
    /// 
    /// use std::fs::File;
    /// use std::io::BufReader;
    /// 
    /// let file = File::create("dummy.json").unwrap();
    /// let reader = BufReader::new(file);
    /// 
    /// let json_reader = JsonReader::new(reader)
    pub fn new(reader: BufReader<T>) -> Self {
        JsonReader { reader, character_buffer: VecDeque::with_capacity(4) }
    }

    /// Create a new [JsonReader] that reads from a given bytes stream
    /// 
    /// #Example
    /// 
    /// use std::io::{BufReader, Cursor};
    /// use json_parser::reader::JsonReader;
    /// let input_json_string = r#"{"key1":"value1", "key2": "value2"}"
    /// 
    /// let json_reader = JsonReader::<Cursor<&'static [u8]>>::from_bytes(input_json_string.as_bytes());
    #[must_use]
    pub fn from_bytes(bytes: &[u8]) -> JsonReader<Cursor<&[u8]>> {
        JsonReader {
            reader: BufReader::new(Cursor::new(bytes)),
            character_buffer: VecDeque::with_capacity(4),
        }
    }
}

impl <T> Iterator for JsonReader<T> where T: Read + Seek {
    
    type Item =  char;

    #[allow(clippy::cast_possible_wrap)]
    fn next(&mut self) -> Option<Self::Item> {
        if !self.character_buffer.is_empty() {
            return self.character_buffer.pop_front();
        }

        let mut utf8_buffer = [0,0,0,0];
        let _ = self.reader.read(&mut utf8_buffer);
        match from_utf8(&utf8_buffer) {
            Ok(string) => {
                self.character_buffer = string.chars().collect();
                self.character_buffer.pop_front()
            }
            Err(error ) => {
                // Read valid bytes, and rewind the buffered reader for
                // the remaining bytes so that they can read again in the 
                // next iteration.
                let valid_bytes = error.valid_up_to();
                let string = from_utf8(&utf8_buffer[..valid_bytes]).unwrap();
                let remaining_bytes = 4 - valid_bytes;

                let _ = self.reader.seek_relative(-(remaining_bytes as i64));
                // Collect the valid characters into character_buffer
                self.character_buffer = string.chars().collect();

                // Return the first character from character_buffer
                self.character_buffer.pop_back()   
            }
        }
    
    }
}

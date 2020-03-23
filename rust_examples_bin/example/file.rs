use std::fs::File;
use std::io::{Write, BufRead, BufReader, Error};

pub fn file_read_line( path : &str ) -> Result<String, Error>
{
    // ? 구분은 'Error'면 Error return. 아니라면 결과값을 받는다.
    let file_open = File::open(path)?;

    let mut buffer = BufReader::new( file_open );
    let mut string_buffer = String::new();
    let size = buffer.read_line( &mut string_buffer )?;

    Ok( string_buffer )
}
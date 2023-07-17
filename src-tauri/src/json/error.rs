#[derive(Debug)]
pub enum FileError {
    CreateError,
    ReadError,
    WriteError,
    ParseError,
}
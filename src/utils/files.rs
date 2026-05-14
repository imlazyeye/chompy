use codespan_reporting::files::{Error, Files, SimpleFile};
use std::ops::Range;

/// Alias around `usize`, which codespan uses as an id for files.
pub type FileId = usize;

/// Holds onto loaded source code to be looked up for diagnostics. Used to interact with codespan
/// via codespan's [Files].
#[derive(Debug, Default)]
pub struct Library(Vec<SimpleFile<String, Box<str>>>);
impl Library {
    /// Create a new Library.
    pub fn new() -> Library {
        Library(Vec::new())
    }

    /// Add a file to the library, returning the handle that can be used to
    /// refer to it again.
    pub fn add(&mut self, name: String, source: impl Into<Box<str>>) -> usize {
        let file_id = self.0.len();
        self.0.push(SimpleFile::new(name, source.into()));
        file_id
    }

    /// Get the file corresponding to the given id.
    ///
    /// ### Errors
    /// Returns an error if the file is not found.
    pub fn get(&self, file_id: usize) -> Result<&SimpleFile<String, Box<str>>, Error> {
        self.0.get(file_id).ok_or(Error::FileMissing)
    }
}
impl<'a> Files<'a> for Library {
    type FileId = FileId;
    type Name = String;
    type Source = &'a str;

    fn name(&self, file_id: usize) -> Result<String, Error> {
        Ok(self.get(file_id)?.name().clone())
    }

    fn source(&self, file_id: usize) -> Result<&str, Error> {
        Ok(self.get(file_id)?.source().as_ref())
    }

    fn line_index(&self, file_id: usize, byte_index: usize) -> Result<usize, Error> {
        self.get(file_id)?.line_index((), byte_index)
    }

    fn line_range(&self, file_id: usize, line_index: usize) -> Result<Range<usize>, Error> {
        self.get(file_id)?.line_range((), line_index)
    }
}

pub struct Context<'a> {
    /// A short message describing where the quoted
    /// text comes from (like a filename)
    source: &'a str,
    /// The Line number in the source
    line_no: usize,
    /// The quoted line
    line: &'a str,
}

impl<'a> Context<'a> {
    pub fn new(source: &'a str, line_no: usize, line: &'a str) -> Self<'a> {
        Self {
            source: source,
            line_no: line_no,
            line: line,
        }
    }

    pub fn source(&'a self) -> &'a str {
        self.source
    }

    pub fn line_no(&self) -> usize {
        self.line_no
    }

    pub fn line(&'a self) -> &'a str {
        self.line
    }

}

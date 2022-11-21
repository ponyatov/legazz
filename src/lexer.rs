/// typed token: type + value
enum Toke<'a> {
    Comment(&'a String),
    Command(&'a String),
    Operator(&'a String),
}

/// source code locator
struct Src<'a> {
    /// file name
    file: &'a str,
    /// binary position (offset)
    pos: usize,
    /// binary size
    len: usize,

    line: usize,
    col: usize,
}

/// positioned token instance
struct Token<'a> {
    token: Toke<'a>,
    loc: Src<'a>,
}

#[derive(Debug)]
pub struct Lexer<'a> {
    /// memory-mapped r/o file (memory buffer)
    mmap: memmap::Mmap,
    /// current reading pointer (index)
    p: usize,
    /// current end pointer: slice `mmap[p..pe]`
    pe: usize,
    /// end of file
    peof: usize,
    /// current file name
    name: &'a String,
}

impl Lexer<'_> {
    fn new(mmap: memmap::Mmap, filename: &String) -> Lexer {
        let size = mmap.len();
        Lexer {
            mmap: mmap,
            p: 0,
            pe: 0,
            peof: size,
            name: filename,
        }
    }

    pub fn token(&self) -> &[u8] {
        &self.mmap[self.p..self.pe + 1]
    }
}

pub fn input(mmap: memmap::Mmap, filename: &String) -> Lexer {
    Lexer::new(mmap, filename)
}

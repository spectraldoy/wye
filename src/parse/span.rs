/// A node in the parse tree of a program is spanned by bytes at certain
/// positions in the text of a program. This struct captures those byte
/// positions along with the node value so that line numbers and other
/// information can be reported with errors.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

pub type OptionSpan = Option<Span>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spanned<T: Clone + PartialEq + Eq> {
    pub value: T,
    pub span: Span,
}

impl<T: Clone + PartialEq + Eq> Spanned<T> {
    pub fn start(&self) -> usize {
        self.span.start
    }

    pub fn end(&self) -> usize {
        self.span.end
    }
}

pub trait UnSpan {
    fn unspanned(&self) -> Self;
}

pub fn unspanned_vec<T: UnSpan>(vec: &Vec<T>) -> Vec<T> {
    vec.iter().map(|elem| elem.unspanned()).collect()
}

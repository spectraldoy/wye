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

    pub fn maybe_new(start: Option<usize>, end: Option<usize>) -> Option<Self> {
        match (start, end) {
            (None, _) | (_, None) => None,
            (Some(start), Some(end)) => Some(Self { start, end })
        }
    }
}

pub type OptionSpan = Option<Span>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spanned<T: Clone + PartialEq + Eq> {
    pub value: T,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptionSpanned<T: Clone + PartialEq + Eq> {
    pub value: T,
    pub span: OptionSpan,
}

impl<T: Clone + PartialEq + Eq> OptionSpanned<T> {
    pub fn start(&self) -> Option<usize> {
        match &self.span {
            None => None,
            Some(span) => Some(span.start)
        }
    }

    pub fn end(&self) -> Option<usize> {
        match &self.span {
            None => None,
            Some(span) => Some(span.end)
        }
    }
}

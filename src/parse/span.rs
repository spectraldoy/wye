/// A node in the parse tree of a program is spanned by bytes at certain
/// positions in the text of a program. This struct captures those byte
/// positions along with the node value so that line numbers and other
/// information can be reported with errors.
use std::iter::ExactSizeIterator;

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

/// Create a span of largest length from a collection of spans.
pub fn widest_span<I>(iter: &I) -> OptionSpan 
where
    I: ExactSizeIterator<Item = Span>
{
    if iter.len() == 0 {
        return None;
    }

    let min_start = usize::MAX;
    let max_end = usize::MIN;
    for span in iter {
        if span.start < min_start {
            min_start = span.start;
        }
        if span.end > max_end {
            max_end = span.end;
        }
    }

    Some(Span::new(min_start, max_end))
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

pub fn unspanned_iter<I, T: UnSpan>(iter: &I) -> Vec<T>
where I: Iterator<Item = T>
{
    iter.iter().map(|elem| elem.unspanned()).collect::<Vec<T>>()
}

pub trait GetSpan {
    fn get_span(&self) -> Span;
}

pub fn get_spans_iter<I, T: GetSpan>(iter: &I) -> Vec<Span>
where I: Iterator<Item = T>    
{
    iter.iter().map(|elem| elem.get_span()).collect::<Vec<T>>()
}

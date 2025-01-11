/// A node in the parse tree of a program is spanned by bytes at certain
/// positions in the text of a program. This struct captures those byte
/// positions along with the node value so that line numbers and other
/// information can be reported with errors.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
pub fn widest_span<I>(seq: &I) -> OptionSpan
where
    I: AsRef<[Span]>,
{
    let mut output = None;
    let mut min_start = usize::MAX;
    let mut max_end = usize::MIN;

    for span in seq.as_ref() {
        if span.start < min_start {
            min_start = span.start;
        }
        if span.end > max_end {
            max_end = span.end;
        }
        output = Some(Span::new(min_start, max_end));
    }

    output
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

pub fn unspanned_seq<I, T: UnSpan>(seq: &I) -> Vec<T>
where
    I: AsRef<[T]> + ?Sized,
{
    seq.as_ref()
        .iter()
        .map(|elem| elem.unspanned())
        .collect::<Vec<T>>()
}

// TODO: once again the spans story could improve
// do I really have to return to this stupid spans problem
// Maybe a global const?

// #[cfg(test)]
// pub const IGNORE_SPANS: bool = true;
// #[cfg(not(test))]
// pub const IGNORE_SPANS: bool = false;
// then use Span::new() everywhere, which defaults
// to usize::MAX, usize::MAX when IGNORE spans is
// true. Then we can have a dummy span constant that
// is basically checked for equality everywhere
// and there is none of this unspanned or unwrapping bs

pub trait GetSpan {
    fn get_span(&self) -> Span;
}

pub fn get_seq_spans<I, T: GetSpan>(seq: &I) -> Vec<Span>
where
    I: AsRef<[T]> + ?Sized,
{
    seq.as_ref()
        .iter()
        .map(|elem| elem.get_span())
        .collect::<Vec<Span>>()
}

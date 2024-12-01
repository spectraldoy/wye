/// A node in the parse tree of a program is spanned by bytes at certain
/// positions in the text of a program. This struct captures those byte
/// positions along with the node value so that line numbers and other
/// information can be reported with errors.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spanned<T>
where
    T: Clone + PartialEq + Eq
{
    pub value: T,
    pub span: Span,
}

impl<T> Spanned<T>
where
    T: Clone + PartialEq + Eq
{
    /// Construct a new spanned value.
    pub fn new(value: T, start: usize, end: usize) -> Self {
        Spanned { value, span: Span { start, end } }
    }
}

/// Reduces clutter
pub fn spanned<T: Clone + PartialEq + Eq>(value: T, span: Span) -> Spanned<T> {
    Spanned { value, span }
}

pub fn get_spanned_value<T: Clone + PartialEq + Eq>(spanned_val: Spanned<T>) -> T {
    spanned_val.value
}
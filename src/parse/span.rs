#[derive(Clone)]
pub struct Spanned<T>
where
    T: Clone,
{
    pub start: usize,
    pub end: usize,
    pub value: T,
}

/// In order to count line numbers:
/// we read the whole file and get the byte positions
/// put them into a btree map which maps byte pos to line number
/// use the lowest match in this btree map to get the line number
/// So spans should use byte positions
///
/// Use codespan_reporting. Then, spans basically only need to have
/// byte start and end information
///
/// Use type holes as types of variables that need to be inferred
/// That way type checking and type inference can proceed simultaneously
/// oh this is epic
/// this is gonna be epic

impl<T> Spanned<T>
where
    T: Clone,
{
    pub fn new(start: usize, end: usize, value: T) -> Self {
        Spanned { start, end, value }
    }
}

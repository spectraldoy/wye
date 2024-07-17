pub struct Spanned<T> {
    pub start: usize,
    pub end: usize,
    pub value: T
}

pub fn make_spanned<T>(start: usize, end: usize, value: T) -> Spanned<T> {
    Spanned {
        start,
        end,
        value
    }
}

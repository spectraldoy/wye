#[derive(Clone)]
pub struct Spanned<T> 
where T: Clone
{
    pub start: usize,
    pub end: usize,
    pub value: T
}

impl<T> Spanned<T>
where T: Clone
{
    pub fn new(start: usize, end: usize, value: T) -> Self {
        Spanned {
            start,
            end,
            value
        }
    }
}

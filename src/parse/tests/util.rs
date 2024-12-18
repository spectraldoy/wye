use ordered_float::OrderedFloat;

/// TODO(WYE-5): document
pub fn to_of64(x: f64) -> OrderedFloat<f64> {
    OrderedFloat::<f64>(x)
}

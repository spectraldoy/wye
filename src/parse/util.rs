use super::span::Spanned;

pub fn spans_overlap(spans: &Vec<(usize, usize)>) -> bool {
    for i in 0..(spans.len() - 1) {
        let (_, cur_end) = spans[i];
        let (nxt_start, _) = spans[i + 1];
        if nxt_start <= cur_end {
            return true;
        }
    }
    return false;
}

// Collects an expression of the form func arg1 arg2 ...arg_n
// into the evaluation order ( ... ( (func arg1) arg2 )...) arg_n)
pub fn collect_function<T: Clone>(
    first_func: Spanned<T>,
    args: Vec<Spanned<T>>,
    mapper: fn(T, T) -> T,
) -> Result<T, &'static str> {
    if args.len() < 1 {
        return Err("Not enough arguments to be a function application");
    }

    // Check spans do not overlap
    // TODO: checking spans do not overlap can be made into its own function
    let mut spans = vec![(first_func.start, first_func.end)];
    args.iter().for_each(|a| {
        spans.push((a.start, a.end))
    });
    if spans_overlap(&spans) {
        return Err("Space required between tokens to constitute a syntactically valid function application");
    }

    // If we reach this point, spans are ok
    if args.len() == 1 {
        Ok(mapper(first_func.value, args[0].value.clone()))
    } else {
        let actual_func = collect_function::<T>(
            first_func,
            args[..args.len() - 1].to_vec(),
            mapper
        );
        match actual_func {
            Ok(func) => Ok(mapper(func, args[args.len() - 1].value.clone())),
            Err(e) => Err(e),
        }
    }
}
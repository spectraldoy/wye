use super::ast;
use super::span::Span;

pub type OptionBox<T> = Option<Box<T>>;

/// TODO(WYE-5)
pub fn spans_overlap(spans: &Vec<Span>) -> Result<(), Span> {
    if spans.len() == 0 {
        return Ok(());
    }

    for i in 0..(spans.len() - 1) {
        let cur_end = spans[i].end;
        let next_start = spans[i + 1].start;
        if next_start <= cur_end {
            return Err(Span {
                start: spans[i].start,
                end: spans[i + 1].end,
            });
        }
    }

    return Ok(());
}

/// TODO(WYE-5)
pub fn flatten_projection(expr: &ast::Expression) -> Result<Vec<ast::Expression>, &'static str> {
    match &expr {
        ast::Expression::Identifier(_, _) => Ok(vec![expr.clone()]),
        ast::Expression::Projection(p, id, _) => {
            let prior_projections = flatten_projection(&p);
            if prior_projections.is_err() {
                return prior_projections;
            }
            let mut projections = prior_projections.unwrap();
            projections.push(ast::Expression::Identifier(id.clone(), None));
            Ok(projections)
        }
        // TODO(WYE-9): Use String for errors
        _ => Err("Called flatten_projection on non-Projection Expression"),
    }
}
#[derive(Debug)]
pub enum RecordMemberness {
    Value,
    Method,
}

// // Collects an expression of the form func arg1 arg2 ...arg_n
// // into the evaluation order ( ... ( (func arg1) arg2 )...) arg_n)
// // TODO: this needs to be better. Also is the mapper necessary?
// // You don't need a type parameter but an instance parameter almost.
// pub fn collect_function<T: Clone>(
//     first_func: Spanned<T>,
//     args: Vec<Spanned<T>>,
//     mapper: fn(T, T) -> T,
// ) -> Result<T, &'static str> {
//     if args.len() < 1 {
//         return Err("Not enough arguments to be a function application");
//     }

//     // Check spans do not overlap

//     let mut spans = vec![(first_func.start, first_func.end)];
//     args.iter().for_each(|a| spans.push((a.start, a.end)));
//     if spans_overlap(&spans) {
//         return Err("Space required between tokens to constitute a syntactically valid function application");
//     }

//     // If we reach this point, spans are ok
//     if args.len() == 1 {
//         Ok(mapper(first_func.value, args[0].value.clone()))
//     } else {
//         let actual_func =
//             collect_function::<T>(first_func, args[..args.len() - 1].to_vec(), mapper);
//         match actual_func {
//             Ok(func) => Ok(mapper(func, args[args.len() - 1].value.clone())),
//             Err(e) => Err(e),
//         }
//     }
// }

use super::ast::Expression;
use super::span::Spanned;

// Collects an expression of the form func arg1 arg2 ...arg_n
// into the evaluation order ( ... ( (func arg1) arg2 )...) arg_n)
pub fn collect_function_application<'a>(
    first_func: Spanned<Expression<'a>>,
    args: Vec<Spanned<Expression<'a>>>
) -> Result<Expression<'a>, &'static str> {
    if args.len() < 1 {
        return Err("Not enough arguments to be a function application");
    }

    // Check spans do not overlap
    let mut spans = vec![(first_func.start, first_func.end)];
    args.iter().for_each(|a| {
        spans.push((a.start, a.end))
    });
    for i in 0..(spans.len() - 1) {
        let (_, cur_end) = spans[i];
        let (nxt_start, _) = spans[i + 1];
        if cur_end == nxt_start {
            return Err("Space required between tokens here, cannot identify if this is a function call or not");
        }
    }

    // If we reach this point, spans are ok
    if args.len() == 1 {
        Ok(Expression::FuncApplication(
            Box::new(first_func.value),
            Box::new(args[0].value.clone())
        ))
    } else {
        let actual_func = collect_function_application(
            first_func,
            args[..args.len() - 1].to_vec()
        );
        match actual_func {
            Ok(func) => Ok(Expression::FuncApplication(
                Box::new(func),
                Box::new(args[args.len() - 1].value.clone())
            )),
            Err(e) => Err(e),
        }
    }
}
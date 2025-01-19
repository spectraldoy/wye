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
pub fn flatten_projection(expr: &ast::Expression) -> Result<Vec<ast::Expression>, String> {
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
        _ => Err(format!(
            "Called flatten_projection on non-Projection Expression"
        )),
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordMemberness {
    Value,
    Method,
}

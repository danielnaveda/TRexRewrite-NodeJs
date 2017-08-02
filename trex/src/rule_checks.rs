use FnvHashMap;
use expressions::inference::{CurrentType, InferenceContext};
use linear_map::LinearMap;
use tesla::{EventTemplate, Rule, TupleDeclaration, TupleType};
use tesla::expressions::*;
use tesla::predicates::*;

// TODO improve error handling and more informative failure,
// or switch completely to a panic!() approach and defer checks to parser

mod aggregate {
    use tesla::TupleDeclaration;
    use tesla::expressions::BasicType;
    use tesla::predicates::Aggregator;

    pub fn get_type(aggregator: &Aggregator,
                    tuple: &TupleDeclaration)
                    -> Result<BasicType, String> {
        match *aggregator {
                Aggregator::Avg(i) => {
                    match tuple.attributes[i].ty {
                        BasicType::Int | BasicType::Float => Some(BasicType::Float),
                        _ => None,
                    }
                }
                Aggregator::Sum(i) |
                Aggregator::Min(i) |
                Aggregator::Max(i) => {
                    match tuple.attributes[i].ty {
                        ref ty @ BasicType::Int |
                        ref ty @ BasicType::Float => Some(ty.clone()),
                        _ => None,
                    }
                }
                Aggregator::Count => Some(BasicType::Int),
            }
            .ok_or("Wrong attribute type in aggregate computation".to_owned())
    }
}

fn type_check_constraints<'a>(constraints: &'a [Expression],
                              ctx: InferenceContext<'a>)
                              -> Result<InferenceContext<'a>, String> {
    constraints.iter().fold(Ok(ctx), |ctx, expr| {
        ctx.and_then(|ctx| {
            ctx.infer_expression(expr).and_then(|ty| {
                if let BasicType::Bool = ty {
                    Ok(ctx)
                } else {
                    Err("Non boolean contraint".to_owned())
                }
            })
        })
    })
}

fn type_check_predicate<'a>(i: usize,
                            pred: &'a Predicate,
                            tuples: &'a FnvHashMap<usize, TupleDeclaration>,
                            ctx: InferenceContext<'a>)
                            -> Result<InferenceContext<'a>, String> {
    tuples.get(&pred.tuple.ty_id)
        .ok_or("Predicate refers to unknown tuple".to_owned())
        .and_then(|tuple| {
            // TODO check that a static predicate refers to a static tuple
            match pred.ty {
                PredicateType::Trigger { ref parameters, .. } |
                PredicateType::Event { ref parameters, .. } |
                PredicateType::OrderedStatic { ref parameters, .. } |
                PredicateType::UnorderedStatic { ref parameters, .. } => {
                    parameters.iter()
                        .enumerate()
                        .fold(Ok(ctx.set_current(CurrentType::Tuple(tuple))),
                              |ctx, (j, param)| {
                                  ctx.and_then(|ctx| {
                                      ctx.infer_expression(&param.expression)
                                          .map(|ty| ctx.add_parameter((i, j), ty))
                                  })
                              })
                        .and_then(|ctx| type_check_constraints(&pred.tuple.constraints, ctx))
                }
                PredicateType::EventAggregate { ref aggregator, ref parameter, .. } |
                PredicateType::StaticAggregate { ref aggregator, ref parameter } => {
                    type_check_constraints(&pred.tuple.constraints,
                                           ctx.set_current(CurrentType::Tuple(tuple)))
                        .and_then(|ctx| {
                            aggregate::get_type(aggregator, tuple).and_then(|ty| {
                                let ctx = ctx.set_current(CurrentType::Aggr(ty));
                                ctx.infer_expression(&parameter.expression)
                                    .map(|ty| ctx.add_parameter((i, 0), ty))
                            })
                        })
                }
                PredicateType::EventNegation { .. } |
                PredicateType::StaticNegation => {
                    type_check_constraints(&pred.tuple.constraints,
                                           ctx.set_current(CurrentType::Tuple(tuple)))
                }
            }
        })
}

fn type_check_template<'a>(template: &'a EventTemplate,
                           tuples: &'a FnvHashMap<usize, TupleDeclaration>,
                           ctx: InferenceContext<'a>)
                           -> Result<InferenceContext<'a>, String> {
    tuples.get(&template.ty_id)
        .ok_or("The rule produce an unknown event".to_owned())
        .and_then(|tuple| {
            if let TupleType::Event = tuple.ty {
                if tuple.attributes.len() == template.attributes.len() {
                    template.attributes
                        .iter()
                        .zip(tuple.attributes.iter().map(|it| &it.ty))
                        .fold(Ok(ctx), |ctx, (expr, ty)| {
                            ctx.and_then(|ctx| {
                                ctx.infer_expression(expr)
                                    .and_then(|res| {
                                        if *ty == res {
                                            Ok(ctx)
                                        } else {
                                            Err("Wrong attribute assignment".to_owned())
                                        }
                                    })
                            })
                        })
                } else {
                    Err("Wrong number of attributes in event template".to_owned())
                }
            } else {
                Err("The rule produce a static tuple".to_owned())
            }
        })
}

// TODO think of a better name or maybe separate funtionality to get params types
pub fn check_rule(rule: &Rule,
                  tuples: &FnvHashMap<usize, TupleDeclaration>)
                  -> Result<LinearMap<(usize, usize), BasicType>, String> {
    rule.predicates
        .iter()
        .enumerate()
        .fold(Ok(InferenceContext::new()), |ctx, (i, pred)| {
            ctx.and_then(|ctx| type_check_predicate(i, pred, tuples, ctx.reset_current()))
        })
        .and_then(|ctx| type_check_constraints(&rule.filters, ctx.reset_current()))
        .and_then(|ctx| type_check_template(&rule.event_template, tuples, ctx.reset_current()))
        // TODO check consuming!
        .map(|ctx| ctx.get_params())
}

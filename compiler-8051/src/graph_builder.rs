use crate::graph::*;
use std::rc::Rc;
use std::fmt::Debug;
use std::ops::FnMut;

pub struct GraphBuilder<T, R: Eq> {
    references: Vec<(R, Rc<GraphBlock<T>>)>,
    building_blocks: Vec<Rc<GraphBlock<T>>>
}

impl<T: Default, R: Eq> GraphBuilder<T, R> {

    pub fn new() -> GraphBuilder<T, R> {
        GraphBuilder {
            references: Vec::new(),
            building_blocks: Vec::new()
        }
    }

    pub fn add(&mut self, data: T, graphif: impl Graphifier<T, R>)  
    {
        self.building_blocks.push({
            let mut result = None;
            let referd = graphif.refered(data);

            fn once_or_panic<K, O>(i: Option<K>, call: impl FnMut(K) -> O, result: &mut Option<O>) {
                if let Some(i) = i {
                    if result.is_some() {
                        panic!("Graphifier gave 2 commandos to 1 T");
                    } else {
                        *result = Some(call(i))
                    }
                }
            }

            let self_reference = |r| {//TODO should we add a GraphBlock::None
                let nop = Rc::new(GraphBlock::Normal(T::default()));
                self.building_blocks.push(nop.clone());
                self.references.push((r, nop.clone()));
                (r, nop)
            };

            once_or_panic(graphif.branch(data), |branch| {
                let (_, rf) = self.references.iter().find(|r| r.0 == branch).or_else(|| {
                    if let Some(referd) = referd {
                        if branch == referd { return Some(&self_reference(referd)); }
                    }
                    None
                }).unwrap();
                GraphBlock::Branch(data, rf)
            }, &mut result);

            once_or_panic(graphif.goto(data), |goto| {

            }, &mut result);

            once_or_panic(graphif.graph_reference(data), |graph| {
                
            }, &mut result);

            let result = Rc::new(result.unwrap_or_else(|| GraphBlock::Normal(data)));
            
            if let Some(r) = referd {
                self.references.push((r, result));
            }

            result
        })
    }

}

pub trait Graphifier<T, R>: Debug {

    fn refered(&self, data: T) -> Option<R>;

    fn branch(&self, data: T) -> Option<R>;

    fn goto(&self, data: T) -> Option<R>;

    fn graph_reference(&self, data: T) -> Option<Rc<dyn GraphPoint<T>>>;

}

struct StartGraphPoint<O, T>(O, GraphLine<T>);

impl<O, T> GraphPoint<T> for StartGraphPoint<O, T> {
    fn line(&self) -> &GraphLine<T> {
        &self.1
    }
}

impl<O, T> StartGraphPoint<O, T> {
    pub fn new(origin: O, line: GraphLine<T>) -> StartGraphPoint<O, T> {
        StartGraphPoint(origin, line)
    }
}

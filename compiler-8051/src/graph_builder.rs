use crate::graph::*;
use std::fmt::Debug;
use std::ops::FnMut;
use std::rc::Rc;

pub struct GraphBuilder<'a, T, R: Eq> {
    references: Vec<(R, usize)>,
    building_blocks: Vec<BuildBlock<'a, T>>,
}

impl<'a, T: 'static + Debug, R: Eq + Debug + 'static> GraphBuilder<'a, T, R> {
    pub fn new() -> GraphBuilder<'a, T, R> {
        GraphBuilder {
            references: Vec::new(),
            building_blocks: Vec::new(),
        }
    }

    pub fn add(&mut self, data: T, graphif: &impl Graphifier<T, R>) {
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

        once_or_panic(
            graphif.branch(data),
            |branch| {
                if Some(branch) == referd {
                    self.building_blocks
                        .push(BuildBlock::Point(&mut |l| Rc::new(GeneralPoint::new(l))))
                }

                GraphBlock::Branch(data, Rc::new(GraphPointReference::new(branch)))
            },
            &mut result,
        );

        once_or_panic(
            graphif.goto(data),
            |goto| {
                if Some(goto) == referd {
                    self.building_blocks
                        .push(BuildBlock::Point(&mut |l| Rc::new(GeneralPoint::new(l))))
                }

                GraphBlock::Branch(data, Rc::new(GraphPointReference::new(goto)))
            },
            &mut result,
        );

        once_or_panic(
            graphif.graph_reference(data),
            |graph| GraphBlock::GraphReference(data, graph),
            &mut result,
        );

        let result = Rc::new(result.unwrap_or_else(|| GraphBlock::Normal(data)));
        self.building_blocks.push(BuildBlock::Block(result));

        if let Some(r) = referd {
            self.references.push((r, self.building_blocks.len() - 1));
        }
    }
}

pub trait Graphifier<T, R>: Debug {
    fn refered(&self, data: T) -> Option<R>;

    fn branch(&self, data: T) -> Option<R>;

    fn goto(&self, data: T) -> Option<R>;

    fn graph_reference(&self, data: T) -> Option<Rc<Graph<T>>>;
}

#[derive(Debug)]
struct StartGraphPoint<O, T>(O, GraphLine<T>);

impl<O: Debug, T: Debug> GraphPoint<T> for StartGraphPoint<O, T> {
    fn line(&self) -> &GraphLine<T> {
        &self.1
    }
}

impl<O, T> StartGraphPoint<O, T> {
    pub fn new(origin: O, line: GraphLine<T>) -> StartGraphPoint<O, T> {
        StartGraphPoint(origin, line)
    }
}

#[derive(Debug)]
struct GeneralPoint<T>(GraphLine<T>);

impl<T> GeneralPoint<T> {
    pub fn new(line: GraphLine<T>) -> GeneralPoint<T> {
        GeneralPoint(line)
    }
}

impl<T: Debug> GraphPoint<T> for GeneralPoint<T> {
    fn line(&self) -> &GraphLine<T> {
        &self.0
    }
}

#[derive(Debug)]
struct GraphPointReference<R>(R);

impl<T: Debug, R: Debug> GraphPoint<T> for GraphPointReference<R> {
    fn line(&self) -> &GraphLine<T> {
        panic!("Trying to acces a graph point reference this shouldn't happen.\n
                This is only used in graph_builder and should be later replaced with correct graphPoints")
    }
}

impl<R: Eq> GraphPointReference<R> {
    fn new(r: R) -> GraphPointReference<R> {
        GraphPointReference(r)
    }

    fn resolve<T>(&self, gb: &GraphBuilder<T, R>, line: GraphLine<T>) -> Rc<dyn GraphPoint<T>> {
        let point = gb
            .references
            .iter()
            .find(|r| r.0 == self.0)
            .map(|r| match gb.building_blocks.get(r.1).unwrap() {
                BuildBlock::Point(r) => Some(r),
                _ => None,
            })
            .flatten()
            .unwrap();

        point(line)
    }
}

enum BuildBlock<'a, T> {
    Block(Rc<GraphBlock<T>>),
    Point(&'a mut dyn FnMut(GraphLine<T>) -> Rc<dyn GraphPoint<T>>),
}

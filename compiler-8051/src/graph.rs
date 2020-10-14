use std::rc::Rc;

#[derive(Debug)]
pub struct Graph<T> {
    start: Rc<dyn GraphPoint<T>>,
}

impl<T> Graph<T> {
    pub fn new<O: 'static>(start: Rc<dyn GraphPoint<T>>) -> Graph<T> {
        Graph {
            start
        }
    }
}

pub trait GraphPoint<T>: std::fmt::Debug {
    fn line(&self) -> &GraphLine<T>;
}

#[derive(Debug)]
pub struct GraphLine<T>(Rc<dyn GraphPoint<T>>, GraphBlockCollection<T>);

pub type GraphBlockCollection<T> = Rc<Vec<Rc<GraphBlock<T>>>>;

#[derive(Debug)]
pub enum GraphBlock<T> {
    Normal(T),
    GraphReference(T, Rc<Graph<T>>),
    Branch(T, Rc<dyn GraphPoint<T>>),
    Goto(T, Rc<dyn GraphPoint<T>>)
}
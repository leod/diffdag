extern crate ndarray;

use std::rc::Rc;
use std::ops::{Add, Sub, Mul, Neg};

use ndarray::{LinalgScalar, Shape};
use ndarray::prelude::*;

#[derive(Clone, Debug)]
pub enum ParameterInit<A: LinalgScalar> {
    Value(ArrayD<A>),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ParameterId(usize);

pub struct ParameterValues<A: LinalgScalar>(Vec<ArrayD<A>>);

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct InputId(usize);

pub struct InputValues<A: LinalgScalar>(Vec<ArrayD<A>>);

#[derive(Clone)]
pub struct NodeRef<A: LinalgScalar>(Rc<Node<A>>);

#[derive(Clone)]
pub enum Node<A: LinalgScalar> {
    Constant(ArrayD<A>),
    Parameter(ParameterId, ParameterInit<A>),
    Input(InputId, Shape<IxDyn>),
    Neg(NodeRef<A>),
    Add(NodeRef<A>, NodeRef<A>),
    Mul(NodeRef<A>, NodeRef<A>),
}

pub struct Graph<A: LinalgScalar> {
    name_path: Vec<String>,
    parameters: Vec<NodeRef<A>>,
    inputs: Vec<NodeRef<A>>,
}

impl<A: LinalgScalar> Graph<A> {
    pub fn new() -> Graph<A> {
        Graph {
            name_path: Vec::new(),
            parameters: Vec::new(),
            inputs: Vec::new(),
        }
    }

    //pub fn input(&mut self, 

    pub fn constant(&mut self, value: ArrayD<A>) -> NodeRef<A> {
        NodeRef::new(Node::Constant(value))
    }

    pub fn 
}

impl<A: LinalgScalar> ParameterInit<A> {
    pub fn init(&self) -> ArrayD<A> {
        match self {
            ParameterInit::Value(value) => value.clone(),
        }
    }
}

impl<A: LinalgScalar> NodeRef<A> {
    pub fn new(node: Node<A>) -> NodeRef<A> {
        NodeRef(Rc::new(node))
    }
}

impl<A: LinalgScalar> Node<A> {
}

impl<'a, A: LinalgScalar> Add for &'a NodeRef<A> {
    type Output = NodeRef<A>;

    fn add(self, other: &'a NodeRef<A>) -> NodeRef<A> {
        NodeRef::new(Node::Add(self.clone(), other.clone()))
    }
}

impl<'a, A: LinalgScalar> Sub for &'a NodeRef<A> {
    type Output = NodeRef<A>;

    fn sub(self, other: &'a NodeRef<A>) -> NodeRef<A> {
        NodeRef::new(Node::Add(self.clone(), -other))
    }
}

impl<'a, A: LinalgScalar> Neg for &'a NodeRef<A> {
    type Output = NodeRef<A>;

    fn neg(self) -> NodeRef<A> {
        NodeRef::new(Node::Neg(self.clone()))
    }
}

impl<'a, A: LinalgScalar> Mul for &'a NodeRef<A> {
    type Output = NodeRef<A>;

    fn mul(self, other: &'a NodeRef<A>) -> NodeRef<A> {
        NodeRef::new(Node::Mul(self.clone(), other.clone()))
    }
}

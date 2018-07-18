extern crate ndarray;

use std::rc::Rc;
use std::ops::{Add, Sub, Mul, Neg};

use ndarray::{LinalgScalar, Shape};
use ndarray::prelude::*;

pub enum ParameterInit<A: LinalgScalar> {
    Value(ArrayD<A>),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ParameterId(usize);

pub struct ParameterDefs<A: LinalgScalar>(Vec<ParameterInit<A>>);

pub struct ParameterValues<A: LinalgScalar>(Vec<ArrayD<A>>);

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct InputId(usize);

pub struct InputDefs(Vec<Shape<IxDyn>>);

pub struct InputValues<A: LinalgScalar>(Vec<ArrayD<A>>);

#[derive(Clone)]
pub struct NodeRef<A: LinalgScalar>(Rc<Node<A>>);

#[derive(Clone)]
pub enum Node<A: LinalgScalar> {
    Constant(ArrayD<A>),
    Parameter(ParameterId),
    Input(InputId),
    Neg(NodeRef<A>),
    Add(NodeRef<A>, NodeRef<A>),
    Mul(NodeRef<A>, NodeRef<A>),
}

impl<A: LinalgScalar> ParameterInit<A> {
    pub fn init(&self) -> ArrayD<A> {
        match self {
            ParameterInit::Value(value) => value.clone(),
        }
    }
}

impl<A: LinalgScalar> ParameterDefs<A> {
    fn create(&mut self, init: ParameterInit<A>) -> ParameterId {
        self.0.push(init);
        ParameterId(self.0.len() - 1)
    }
}

impl<A: LinalgScalar> ParameterValues<A> {
    pub fn init(defs: &ParameterDefs<A>) -> Self {
        ParameterValues(defs.0.iter().map(|init| init.init()).collect())
    }
}

impl InputDefs {
    fn create(&mut self, shape: Shape<IxDyn>) -> InputId {
        self.0.push(shape);
        InputId(self.0.len() - 1)
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

pub fn constant<A: LinalgScalar>(value: ArrayD<A>) -> NodeRef<A> {
    NodeRef::new(Node::Constant(value))
}

pub fn parameter<A: LinalgScalar>(
    defs: &mut ParameterDefs<A>,
    init: ParameterInit<A>,
) -> NodeRef<A> {
    NodeRef::new(Node::Parameter(defs.create(init)))
}

pub fn input<A: LinalgScalar>(defs: &mut InputDefs, shape: Shape<IxDyn>) -> NodeRef<A> {
    NodeRef::new(Node::Input(defs.create(shape)))
}

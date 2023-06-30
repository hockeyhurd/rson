use crate::rnodes::rnode::{EnumNodeType, RNode};
use crate::visitor::visitor::Visitor;

use std::cell::RefCell;
use std::rc::Rc;

pub struct RNodeDouble
{
    pub value: f64,
}

impl RNodeDouble
{
    pub fn new(value: f64) -> Self
    {
        Self { value }
    }
}

impl RNode for RNodeDouble
{
    fn get_node_type(&self) -> EnumNodeType
    {
        return EnumNodeType::DOUBLE;
    }

    fn accept(&mut self, visitor: Rc<RefCell<dyn Visitor>>)
    {
        visitor.borrow_mut().visit_double(self);
    }
}


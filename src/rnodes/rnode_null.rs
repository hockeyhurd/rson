use crate::rnodes::rnode::{EnumNodeType, RNode};
use crate::visitor::visitor::Visitor;

use std::cell::RefCell;
use std::rc::Rc;

pub struct RNodeNull;

impl RNodeNull
{
    pub fn new() -> Self
    {
        Self {}
    }
}

impl RNode for RNodeNull
{
    fn get_node_type(&self) -> EnumNodeType
    {
        return EnumNodeType::NULL;
    }

    fn accept(&mut self, visitor: Rc<RefCell<dyn Visitor>>)
    {
        visitor.borrow_mut().visit_null(self);
    }
}


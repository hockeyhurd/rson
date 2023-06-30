use crate::rnodes::rnode::{EnumNodeType, RNode};
use crate::visitor::visitor::Visitor;

use std::cell::RefCell;
use std::rc::Rc;

pub struct RNodeBool
{
    pub value: bool,
}

impl RNodeBool
{
    pub fn new(value: bool) -> Self
    {
        Self { value }
    }
}

impl RNode for RNodeBool
{
    fn get_node_type(&self) -> EnumNodeType
    {
        return EnumNodeType::BOOL;
    }

    fn accept(&mut self, visitor: Rc<RefCell<dyn Visitor>>)
    {
        visitor.borrow_mut().visit_bool(self);
    }
}


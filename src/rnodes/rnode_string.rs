use crate::rnodes::rnode::{EnumNodeType, RNode};
use crate::visitor::visitor::Visitor;

use std::cell::RefCell;
use std::rc::Rc;

pub struct RNodeString
{
    value: String,
}

impl RNodeString
{
    #[allow(dead_code)]
    pub fn new_copy(value: &String) -> Self
    {
        Self { value: value.clone() }
    }

    #[allow(dead_code)]
    pub fn new_move(value: String) -> Self
    {
        Self { value }
    }

    #[allow(dead_code)]
    pub fn get_value(&self) -> &String
    {
        return &self.value;
    }

    #[allow(dead_code)]
    pub fn set_value_copy(&mut self, value: &String)
    {
        self.value = value.clone();
    }

    #[allow(dead_code)]
    pub fn set_value_move(&mut self, value: String)
    {
        self.value = value;
    }
}

impl RNode for RNodeString
{
    fn get_node_type(&self) -> EnumNodeType
    {
        return EnumNodeType::STRING;
    }

    fn accept(&mut self, visitor: Rc<RefCell<dyn Visitor>>)
    {
        visitor.borrow_mut().visit_string(self);
    }
}


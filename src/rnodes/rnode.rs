use downcast_rs::Downcast;

use crate::visitor::visitor::Visitor;

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq)]
pub enum EnumNodeType
{
    ARRAY = 0, BOOL, DOUBLE, NULL, OBJECT, STRING
}

pub trait RNode : Downcast
{
    fn get_node_type(&self) -> EnumNodeType;
    fn accept(&mut self, visitor: Rc<RefCell<dyn Visitor>>);
}

impl_downcast!(RNode);


use downcast_rs::Downcast;

use crate::visitor::visitor::Visitor;

use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::{self, Display};

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

impl Display for EnumNodeType
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            EnumNodeType::ARRAY => write!(f, "ARRAY"),
            EnumNodeType::BOOL => write!(f, "BOOL"),
            EnumNodeType::DOUBLE => write!(f, "DOUBLE"),
            EnumNodeType::NULL => write!(f, "NULL"),
            EnumNodeType::OBJECT => write!(f, "OBJECT"),
            EnumNodeType::STRING => write!(f, "STRING"),
        }
    }
}

impl_downcast!(RNode);


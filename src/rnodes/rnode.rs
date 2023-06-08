use downcast_rs::Downcast;

use std::any::Any;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum EnumNodeType
{
    ARRAY = 0, BOOL, DOUBLE, OBJECT, STRING
}

pub trait Visitor
{
    fn visit_array(&mut self, node: &mut Rc<dyn Visitor>) -> dyn Any;
    fn visit_bool(&mut self, node: &mut Rc<dyn Visitor>) -> dyn Any;
    fn visit_char(&mut self, node: &mut Rc<dyn Visitor>) -> dyn Any;
    fn visit_object(&mut self, node: &mut Rc<dyn Visitor>) -> dyn Any;
    fn visit_string(&mut self, node: &mut Rc<dyn Visitor>) -> dyn Any;
}

pub trait RNode : Downcast
{
    fn get_node_type(&self) -> EnumNodeType;

    /*fn accept(&mut self, visitor: Rc<dyn Visitor>) -> dyn Any
    {
        match self.get_node_type()
        {
            EnumNodeType::ARRAY => { return visitor.visit_array(self); },
            EnumNodeType::BOOL => { return visitor.visit_bool(self); },
            EnumNodeType::DOUBLE => { return visitor.visit_double(self); },
            EnumNodeType::OBJECT => { return visitor.visit_object(self); },
            EnumNodeType::STRING => { return visitor.visit_string(self); },
            _ => { panic!(); },
        }
    }*/
}

impl_downcast!(RNode);

pub struct NodeBool
{
    pub value: bool,
}

impl NodeBool
{
    pub fn new(value: bool) -> Self
    {
        Self { value }
    }
}

impl RNode for NodeBool
{
    fn get_node_type(&self) -> EnumNodeType
    {
        return EnumNodeType::BOOL;
    }
}


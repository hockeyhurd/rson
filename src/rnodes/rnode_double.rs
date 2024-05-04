use crate::rnodes::rnode::{EnumNodeType, RNode};
use crate::visitor::visitor::Visitor;

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

    fn accept(&self, visitor: &dyn Visitor)
    {
        visitor.visit_double(self);
    }
}


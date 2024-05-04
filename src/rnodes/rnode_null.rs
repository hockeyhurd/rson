use crate::rnodes::rnode::{EnumNodeType, RNode};
use crate::visitor::visitor::Visitor;

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

    fn accept(&self, visitor: &dyn Visitor)
    {
        visitor.visit_null(self);
    }
}


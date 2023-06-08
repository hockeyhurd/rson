use crate::rnodes::rnode::{EnumNodeType, RNode};

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
        return EnumNodeType::BOOL;
    }
}


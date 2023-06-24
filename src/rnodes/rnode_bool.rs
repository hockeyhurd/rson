use crate::rnodes::rnode::{EnumNodeType, RNode};

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
}


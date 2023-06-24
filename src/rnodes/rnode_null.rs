use crate::rnodes::rnode::{EnumNodeType, RNode};

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
}


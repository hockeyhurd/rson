use crate::rnodes::rnode::{EnumNodeType, RNode};

use std::ops::Deref;
use std::rc::Rc;

pub struct RNodeArray
{
    arr: Vec<Rc<dyn RNode>>,
}

impl RNodeArray
{
    #[allow(dead_code)]
    pub fn new(arr: Vec<Rc<dyn RNode>>) -> Self
    {
        Self { arr }
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool
    {
        return self.arr.is_empty();
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize
    {
        return self.arr.len();
    }

    #[allow(dead_code)]
    pub fn get(&self, index: usize) -> Option<Rc<dyn RNode>>
    {
        if index < self.arr.len()
        {
            return Some(Rc::clone(&self.arr[index]));
        }

        return None;
    }
}

impl RNode for RNodeArray
{
    fn get_node_type(&self) -> EnumNodeType
    {
        return EnumNodeType::ARRAY;
    }
}

impl Deref for RNodeArray
{
    type Target = Vec<Rc<dyn RNode>>;

    fn deref(&self) -> &Self::Target
    {
        return &self.arr;
    }
}


use crate::rnodes::rnode::{EnumNodeType, RNode};
use crate::visitor::visitor::Visitor;

use std::ops::Deref;
use std::cell::RefCell;
use std::rc::Rc;

pub struct RNodeArray
{
    arr: Vec<Rc<dyn RNode>>,
}

impl RNodeArray
{
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

    fn accept(&mut self, visitor: Rc<RefCell<dyn Visitor>>)
    {
        visitor.borrow_mut().visit_array(self);
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


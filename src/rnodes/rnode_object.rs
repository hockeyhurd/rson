use crate::rnodes::rnode::{EnumNodeType, RNode};
use crate::visitor::visitor::Visitor;

use std::collections::BTreeMap;
use std::ops::Deref;
use std::rc::Rc;

pub struct RNodeObject
{
    map: BTreeMap<String, Rc<dyn RNode>>,
}

impl RNodeObject
{
    pub fn new(map: BTreeMap<String, Rc<dyn RNode>>) -> Self
    {
        Self { map }
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool
    {
        return self.map.is_empty();
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize
    {
        return self.map.len();
    }

    #[allow(dead_code)]
    pub fn add_copy(mut self, key: &String, value: Rc<dyn RNode>) -> Self
    {
        self.map.insert(key.clone(), value);
        self
    }

    #[allow(dead_code)]
    pub fn add_move(mut self, key: String, value: Rc<dyn RNode>) -> Self
    {
        self.map.insert(key, value);
        self
    }

    #[allow(dead_code)]
    pub fn get(&self, key: &String) -> Option<Rc<dyn RNode>>
    {
        let opt_value = self.map.get(key);

        match opt_value
        {
            Some(value) => { return Some(Rc::clone(&value)); },
            None => { return None; },
        }
    }

    #[allow(dead_code)]
    pub fn get_mut(&mut self, key: &String) -> Option<Rc<dyn RNode>>
    {
        let opt_value = self.map.get_mut(key);

        match opt_value
        {
            Some(value) => { return Some(Rc::clone(&value)); },
            None => { return None; },
        }
    }

    pub fn get_map(&self) -> &BTreeMap<String, Rc<dyn RNode>>
    {
        return &self.map;
    }
}

impl RNode for RNodeObject
{
    fn get_node_type(&self) -> EnumNodeType
    {
        return EnumNodeType::OBJECT;
    }

    fn accept(&self, visitor: &dyn Visitor)
    {
        visitor.visit_object(self);
    }
}

impl Deref for RNodeObject
{
    type Target = BTreeMap<String, Rc<dyn RNode>>;

    fn deref(&self) -> &Self::Target
    {
        return &self.map;
    }
}


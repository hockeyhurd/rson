use crate::rnodes::rnode::{EnumNodeType, RNode};

use std::collections::BTreeMap;
use std::ops::Deref;
use std::rc::Rc;

pub struct RNodeObject
{
    map: BTreeMap<String, Rc<dyn RNode>>,
}

impl RNodeObject
{
    #[allow(dead_code)]
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
    pub fn get(&self, key: &String) -> Option<Rc<dyn RNode>>
    {
        let opt_value = self.map.get(key);

        match opt_value
        {
            Some(value) => { return Some(Rc::clone(&value)); },
            None => { return None; },
        }
    }
}

impl RNode for RNodeObject
{
    fn get_node_type(&self) -> EnumNodeType
    {
        return EnumNodeType::OBJECT;
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


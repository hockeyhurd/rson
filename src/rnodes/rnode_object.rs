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

#[cfg(test)]
mod tests
{
    use super::RNodeObject;
    use crate::rnodes::rnode::{EnumNodeType, RNode};
    use crate::rnodes::rnode_double::RNodeDouble;
    use std::collections::BTreeMap;
    use std::rc::Rc;

    #[test]
    fn create_object_from_builder()
    {
        let key0 = String::from("key0");
        let key1 = String::from("key1");
        let val0 = Rc::new(RNodeDouble::new(123.45));
        let val1 = Rc::new(RNodeDouble::new(-123.45));

        let mut obj = RNodeObject::new(BTreeMap::<String, Rc<dyn RNode>>::new());
        obj = obj.add_copy(&key0, val0.clone()).add_copy(&key1, val1.clone());
        assert_eq!(obj.get_node_type(), EnumNodeType::OBJECT);

        let opt_val0 = obj.get(&key0);
        assert!(opt_val0.is_some());
        let node_double = opt_val0.unwrap().downcast_rc::<RNodeDouble>().map_err(|_| "Shouldn't happen").unwrap();
        assert_eq!(node_double.value, val0.value);

        let opt_val1 = obj.get(&key1);
        assert!(opt_val1.is_some());
        let node_double = opt_val1.unwrap().downcast_rc::<RNodeDouble>().map_err(|_| "Shouldn't happen").unwrap();
        assert_eq!(node_double.value, val1.value);
    }
}


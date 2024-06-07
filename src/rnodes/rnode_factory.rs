use crate::rnodes::rnode::{EnumNodeType, RNode};
use crate::rnodes::rnode_array::RNodeArray;
use crate::rnodes::rnode_bool::RNodeBool;
use crate::rnodes::rnode_double::RNodeDouble;
use crate::rnodes::rnode_null::RNodeNull;
use crate::rnodes::rnode_object::RNodeObject;
use crate::rnodes::rnode_string::RNodeString;

use std::collections::BTreeMap;
use std::rc::Rc;

#[allow(unused)]
pub fn create_node(node_type: EnumNodeType) -> Result<Rc<dyn RNode>, String>
{
    match node_type
    {
        EnumNodeType::ARRAY => { return Ok(Rc::new(RNodeArray::new(vec![]))); },
        EnumNodeType::BOOL => { return Ok(Rc::new(RNodeBool::new(false))); },
        EnumNodeType::DOUBLE => { return Ok(Rc::new(RNodeDouble::new(0.0))); },
        EnumNodeType::NULL => { return Ok(Rc::new(RNodeNull::new())); },
        EnumNodeType::OBJECT => { return Ok(Rc::new(RNodeObject::new(BTreeMap::<String, Rc<dyn RNode>>::new()))); },
        EnumNodeType::STRING => { return Ok(Rc::new(RNodeString::new_move(String::new()))); },
        _ => { return Err(String::from("Could not find EnumNodeType")); }
    }
}

#[cfg(test)]
mod tests
{
    use crate::rnodes::rnode::EnumNodeType;
    use crate::rnodes::rnode_array::RNodeArray;
    use crate::rnodes::rnode_bool::RNodeBool;
    use crate::rnodes::rnode_double::RNodeDouble;
    use crate::rnodes::rnode_object::RNodeObject;
    use crate::rnodes::rnode_string::RNodeString;
    use super::create_node;

    #[test]
    fn create_array()
    {
        const NODE_TYPE: EnumNodeType = EnumNodeType::ARRAY;
        let result = create_node(NODE_TYPE);
        assert!(result.is_ok());

        let node = result.unwrap();
        assert_eq!(node.get_node_type(), NODE_TYPE);

        let node_array = node.downcast_rc::<RNodeArray>().map_err(|_| "Shouldn't happen").unwrap();
        assert_eq!(node_array.len(), 0);
    }

    #[test]
    fn create_bool()
    {
        const NODE_TYPE: EnumNodeType = EnumNodeType::BOOL;
        let result = create_node(NODE_TYPE);
        assert!(result.is_ok());

        let node = result.unwrap();
        assert_eq!(node.get_node_type(), NODE_TYPE);

        let node_bool = node.downcast_rc::<RNodeBool>().map_err(|_| "Shouldn't happen").unwrap();
        assert!(!node_bool.value);
    }

    #[test]
    fn create_double()
    {
        const NODE_TYPE: EnumNodeType = EnumNodeType::DOUBLE;
        let result = create_node(NODE_TYPE);
        assert!(result.is_ok());

        let node = result.unwrap();
        assert_eq!(node.get_node_type(), NODE_TYPE);

        let node_double = node.downcast_rc::<RNodeDouble>().map_err(|_| "Shouldn't happen").unwrap();
        assert_eq!(node_double.value, 0.0);
    }

    #[test]
    fn create_null()
    {
        const NODE_TYPE: EnumNodeType = EnumNodeType::NULL;
        let result = create_node(NODE_TYPE);
        assert!(result.is_ok());

        let node = result.unwrap();
        assert_eq!(node.get_node_type(), NODE_TYPE);
    }

    #[test]
    fn create_object()
    {
        const NODE_TYPE: EnumNodeType = EnumNodeType::OBJECT;
        let result = create_node(NODE_TYPE);
        assert!(result.is_ok());

        let node = result.unwrap();
        assert_eq!(node.get_node_type(), NODE_TYPE);

        let node_object = node.downcast_rc::<RNodeObject>().map_err(|_| "Shouldn't happen").unwrap();
        assert!(node_object.is_empty());
    }

    #[test]
    fn create_string()
    {
        const NODE_TYPE: EnumNodeType = EnumNodeType::STRING;
        let result = create_node(NODE_TYPE);
        assert!(result.is_ok());

        let node = result.unwrap();
        assert_eq!(node.get_node_type(), NODE_TYPE);

        let node_string = node.downcast_rc::<RNodeString>().map_err(|_| "Shouldn't happen").unwrap();
        assert!(node_string.get_value().is_empty());
    }
}


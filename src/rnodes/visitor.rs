use super::rnode_array::RNodeArray;
use super::rnode_bool::RNodeBool;
use super::rnode_double::RNodeDouble;
use super::rnode_null::RNodeNull;
use super::rnode_object::RNodeObject;
use super::rnode_string::RNodeString;

pub trait Visitor
{
    fn visit_array(&mut self, node: &mut RNodeArray);
    fn visit_bool(&mut self, node: &mut RNodeBool);
    fn visit_double(&mut self, node: &mut RNodeDouble);
    fn visit_null(&mut self, node: &mut RNodeNull);
    fn visit_object(&mut self, node: &mut RNodeObject);
    fn visit_string(&mut self, node: &mut RNodeString);
}


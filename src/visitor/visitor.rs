use crate::rnodes::rnode_array::RNodeArray;
use crate::rnodes::rnode_bool::RNodeBool;
use crate::rnodes::rnode_double::RNodeDouble;
use crate::rnodes::rnode_null::RNodeNull;
use crate::rnodes::rnode_object::RNodeObject;
use crate::rnodes::rnode_string::RNodeString;

pub trait Visitor
{
    fn visit_array(&mut self, node: &mut RNodeArray);
    fn visit_bool(&mut self, node: &mut RNodeBool);
    fn visit_double(&mut self, node: &mut RNodeDouble);
    fn visit_null(&mut self, node: &mut RNodeNull);
    fn visit_object(&mut self, node: &mut RNodeObject);
    fn visit_string(&mut self, node: &mut RNodeString);
}


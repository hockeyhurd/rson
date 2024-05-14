use crate::rnodes::rnode_array::RNodeArray;
use crate::rnodes::rnode_bool::RNodeBool;
use crate::rnodes::rnode_double::RNodeDouble;
use crate::rnodes::rnode_null::RNodeNull;
use crate::rnodes::rnode_object::RNodeObject;
use crate::rnodes::rnode_string::RNodeString;

pub trait Visitor
{
    fn visit_array(&self, node: &RNodeArray);
    fn visit_bool(&self, node: &RNodeBool);
    fn visit_double(&self, node: &RNodeDouble);
    fn visit_null(&self, node: &RNodeNull);
    fn visit_object(&self, node: &RNodeObject);
    fn visit_string(&self, node: &RNodeString);
}


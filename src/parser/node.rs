use std::rc::Rc;

use downcast_rs::Downcast;

#[allow(dead_code)]
pub enum EnumNodeType
{
    ARRAY = 0, BOOL, DOUBLE, OBJECT, STRING
}

pub trait NodeTrait : Downcast
{
    fn get_type(&self) -> EnumNodeType;
    fn as_array(&self) -> Result<Vec<Rc<dyn NodeTrait>>, String>;
    fn as_bool(&self) -> Result<bool, String>;
    fn as_double(&self) -> Result<f64, String>;
    fn as_object(&self) -> Result<Rc<dyn NodeTrait>, String>;
    fn as_string(&self) -> Result<String, String>;
    fn is_array(&self) -> bool;
    fn is_bool(&self) -> bool;
    fn is_double(&self) -> bool;
    fn is_object(&self) -> bool;
    fn is_string(&self) -> bool;
}

impl_downcast!(NodeTrait);

pub struct NodeDouble
{
    value: f64,
}

impl NodeDouble
{
    #[allow(dead_code)]
    pub fn new(value: f64) -> Self
    {
        Self { value }
    }

    #[allow(dead_code)]
    pub fn get_value(&self) -> f64
    {
        return self.value;
    }

    #[allow(dead_code)]
    pub fn set_value(&mut self, value: f64)
    {
        self.value = value;
    }
}

impl NodeTrait for NodeDouble
{
    fn get_type(&self) -> EnumNodeType
    {
        return EnumNodeType::DOUBLE;
    }

    fn as_array(&self) -> Result<Vec<Rc<dyn NodeTrait>>, String>
    {
        return Err("NodeDouble is not a NodeArray".to_string());
    }

    fn as_bool(&self) -> Result<bool, String>
    {
        return Err("NodeDouble is not a NodeBool".to_string());
    }

    fn as_double(&self) -> Result<f64, String>
    {
        return Ok(self.value);
    }

    fn as_object(&self) -> Result<Rc<dyn NodeTrait>, String>
    {
        return Err("NodeDouble is not a NodeObject".to_string());
    }

    fn as_string(&self) -> Result<String, String>
    {
        return Err("NodeDouble is not a NodeString".to_string());
    }

    fn is_array(&self) -> bool
    {
        false
    }

    fn is_bool(&self) -> bool
    {
        false
    }

    fn is_double(&self) -> bool
    {
        true
    }

    fn is_object(&self) -> bool
    {
        false
    }

    fn is_string(&self) -> bool
    {
        false
    }
}


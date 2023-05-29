use std::rc::Rc;

use downcast_rs::Downcast;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum EnumTokenType
{
    ARRAY = 0, BOOL, DOUBLE, OBJECT, STRING, SYMBOL
}

pub trait TokenTrait : Downcast
{
    fn get_type(&self) -> EnumTokenType;
    fn as_array(&self) -> Result<Vec<Rc<dyn TokenTrait>>, String>;
    fn as_bool(&self) -> Result<bool, String>;
    fn as_double(&self) -> Result<f64, String>;
    fn as_object(&self) -> Result<Rc<dyn TokenTrait>, String>;
    fn as_string(&self) -> Result<&String, String>;
    fn as_symbol(&self) -> Result<&String, String>;
    fn is_array(&self) -> bool;
    fn is_bool(&self) -> bool;
    fn is_double(&self) -> bool;
    fn is_object(&self) -> bool;
    fn is_string(&self) -> bool;
    fn is_symbol(&self) -> bool;
}

impl_downcast!(TokenTrait);


use downcast_rs::Downcast;

#[derive(Debug, PartialEq)]
pub enum EnumTokenType
{
    BOOL = 0, CHAR, DOUBLE, NULL, STRING, SYMBOL
}

pub trait TokenTrait : Downcast
{
    fn get_type(&self) -> EnumTokenType;
    fn as_bool(&self) -> Result<bool, String>;
    #[allow(dead_code)]
    fn as_char(&self) -> Result<char, String>;
    fn as_double(&self) -> Result<f64, String>;
    fn as_string(&self) -> Result<&String, String>;
    fn as_symbol(&self) -> Result<&String, String>;
    fn is_bool(&self) -> bool;
    #[allow(dead_code)]
    fn is_char(&self) -> bool;
    fn is_double(&self) -> bool;
    fn is_null(&self) -> bool;
    fn is_string(&self) -> bool;
    fn is_symbol(&self) -> bool;
}

impl_downcast!(TokenTrait);


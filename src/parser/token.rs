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

pub struct TokenDouble
{
    value: f64,
}

pub struct TokenSymbol
{
    symbol: String,
}

impl TokenDouble
{
    #[allow(dead_code)]
    pub fn new(value: f64) -> Self
    {
        Self { value }
    }

    #[allow(dead_code)]
    pub fn get_value(&self) -> f64
    {
        return self.value.into();
    }

    #[allow(dead_code)]
    pub fn set_value(&mut self, value: f64)
    {
        self.value = value.into();
    }
}

impl TokenTrait for TokenDouble
{
    fn get_type(&self) -> EnumTokenType
    {
        return EnumTokenType::DOUBLE;
    }

    fn as_array(&self) -> Result<Vec<Rc<dyn TokenTrait>>, String>
    {
        return Err("TokenDouble is not a TokenArray".to_string());
    }

    fn as_bool(&self) -> Result<bool, String>
    {
        return Err("TokenDouble is not a TokenBool".to_string());
    }

    fn as_double(&self) -> Result<f64, String>
    {
        return Ok(self.value);
    }

    fn as_object(&self) -> Result<Rc<dyn TokenTrait>, String>
    {
        return Err("TokenDouble is not a TokenObject".to_string());
    }

    fn as_string(&self) -> Result<&String, String>
    {
        return Err("TokenDouble is not a TokenString".to_string());
    }

    fn as_symbol(&self) -> Result<&String, String>
    {
        return Err("TokenDouble is not a TokenSymbol".to_string());
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

    fn is_symbol(&self) -> bool
    {
        false
    }
}

impl TokenSymbol
{
    #[allow(dead_code)]
    pub fn new(symbol: String) -> Self
    {
        Self { symbol }
    }

    #[allow(dead_code)]
    pub fn get_symbol(&self) -> &String
    {
        return &self.symbol;
    }

    #[allow(dead_code)]
    pub fn get_sybmol_mut(&mut self) -> &mut String
    {
        return &mut self.symbol;
    }
}

impl TokenTrait for TokenSymbol
{
    fn get_type(&self) -> EnumTokenType
    {
        return EnumTokenType::SYMBOL;
    }

    fn as_array(&self) -> Result<Vec<Rc<dyn TokenTrait>>, String>
    {
        return Err("TokenSymbol is not a TokenArray".to_string());
    }

    fn as_bool(&self) -> Result<bool, String>
    {
        return Err("TokenSymbol is not a TokenBool".to_string());
    }

    fn as_double(&self) -> Result<f64, String>
    {
        return Err("TokenSymbol is not a TokenDouble".to_string());
    }

    fn as_object(&self) -> Result<Rc<dyn TokenTrait>, String>
    {
        return Err("TokenSymbol is not a TokenObject".to_string());
    }

    fn as_string(&self) -> Result<&String, String>
    {
        return Err("TokenSymbol is not a TokenString".to_string());
    }

    fn as_symbol(&self) -> Result<&String, String>
    {
        return Ok(&self.get_symbol());
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

    fn is_symbol(&self) -> bool
    {
        true
    }
}

#[cfg(test)]
mod tests
{
    use crate::parser::token::{EnumTokenType, TokenTrait, TokenDouble};

    #[test]
    fn create_token_double()
    {
        let mut in_value = 42.0;
        let mut token = TokenDouble::new(in_value);
        assert_eq!(token.get_type(), EnumTokenType::DOUBLE);
        assert_eq!(token.get_value(), in_value);

        in_value = 101.0;
        token.set_value(in_value);
        assert_eq!(token.get_value(), in_value);
    }
}


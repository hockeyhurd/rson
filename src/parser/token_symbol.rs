use std::rc::Rc;

use crate::parser::token::{EnumTokenType, TokenTrait};

pub struct TokenSymbol
{
    symbol: String,
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

    #[allow(dead_code)]
    pub fn set_symbol_copy(&mut self, symbol: &String)
    {
        self.symbol = symbol.clone();
    }

    #[allow(dead_code)]
    pub fn set_symbol_move(&mut self, symbol: String)
    {
        self.symbol = symbol;
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
    use crate::parser::token::{EnumTokenType, TokenTrait};
    use crate::parser::token_symbol::TokenSymbol;

    #[test]
    fn create_token_symbol()
    {
        let mut in_value = String::from("Hello, world!");
        let mut token = TokenSymbol::new(in_value.clone());
        assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
        assert_eq!(token.get_symbol(), &in_value);

        in_value += " Hello, again, world!";
        token.set_symbol_copy(&in_value);
        assert_eq!(token.get_symbol(), &in_value);
    }
}



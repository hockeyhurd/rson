use crate::parser::token::{EnumTokenType, TokenTrait};

pub struct TokenNull;

impl TokenNull
{
    #[allow(dead_code)]
    pub fn new() -> Self
    {
        Self {}
    }
}

impl TokenTrait for TokenNull
{
    fn get_type(&self) -> EnumTokenType
    {
        return EnumTokenType::NULL;
    }

    fn as_bool(&self) -> Result<bool, String>
    {
        return Err("TokenNull is not a TokenBool".to_string());
    }

    fn as_char(&self) -> Result<char, String>
    {
        return Err("TokenNull is not a TokenChar".to_string());
    }

    fn as_double(&self) -> Result<f64, String>
    {
        return Err("TokenNull is not a TokenDouble".to_string());
    }

    fn as_string(&self) -> Result<&String, String>
    {
        return Err("TokenNull is not a TokenString".to_string());
    }

    fn as_symbol(&self) -> Result<&String, String>
    {
        return Err("TokenNull is not a TokenSymbol".to_string());
    }

    fn is_bool(&self) -> bool
    {
        false
    }

    fn is_char(&self) -> bool
    {
        false
    }

    fn is_double(&self) -> bool
    {
        true
    }

    fn is_null(&self) -> bool
    {
        true
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

#[cfg(test)]
mod tests
{
    use crate::parser::token::{EnumTokenType, TokenTrait};
    use crate::parser::token_null::TokenNull;

    #[test]
    fn create_token_null()
    {
        let token = TokenNull::new();
        assert_eq!(token.get_type(), EnumTokenType::NULL);
    }
}


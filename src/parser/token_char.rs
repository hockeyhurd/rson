use crate::parser::token::{EnumTokenType, TokenTrait};

pub struct TokenChar
{
    value: char,
}

impl TokenChar
{
    #[allow(dead_code)]
    pub fn new(value: char) -> Self
    {
        Self { value }
    }

    #[allow(dead_code)]
    pub fn get_value(&self) -> char
    {
        return self.value.into();
    }

    #[allow(dead_code)]
    pub fn set_value(&mut self, value: char)
    {
        self.value = value.into();
    }
}

impl TokenTrait for TokenChar
{
    fn get_type(&self) -> EnumTokenType
    {
        return EnumTokenType::CHAR;
    }

    fn as_bool(&self) -> Result<bool, String>
    {
        return Err("TokenChar is not a TokenBool".to_string());
    }

    fn as_char(&self) -> Result<char, String>
    {
        return Ok(self.value);
    }

    fn as_double(&self) -> Result<f64, String>
    {
        return Err("TokenChar is not a TokenDouble".to_string());
    }

    fn as_string(&self) -> Result<&String, String>
    {
        return Err("TokenChar is not a TokenString".to_string());
    }

    fn as_symbol(&self) -> Result<&String, String>
    {
        return Err("TokenChar is not a TokenSymbol".to_string());
    }

    fn is_bool(&self) -> bool
    {
        false
    }

    fn is_char(&self) -> bool
    {
        true
    }

    fn is_double(&self) -> bool
    {
        false
    }

    fn is_null(&self) -> bool
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

#[cfg(test)]
mod tests
{
    use crate::parser::token::{EnumTokenType, TokenTrait};
    use crate::parser::token_char::TokenChar;

    #[test]
    fn create_token_char()
    {
        let mut in_value = 'a';
        let mut token = TokenChar::new(in_value);
        assert_eq!(token.get_type(), EnumTokenType::CHAR);
        assert_eq!(token.get_value(), in_value);

        in_value = 'A';
        token.set_value(in_value);
        assert_eq!(token.get_value(), in_value);
    }
}



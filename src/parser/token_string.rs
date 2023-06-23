use crate::parser::token::{EnumTokenType, TokenTrait};

pub struct TokenString
{
    value: String,
}

impl TokenString
{
    #[allow(dead_code)]
    pub fn new(value: String) -> Self
    {
        Self { value }
    }

    #[allow(dead_code)]
    pub fn get_value(&self) -> &String
    {
        return &self.value;
    }

    #[allow(dead_code)]
    pub fn get_value_mut(&mut self) -> &mut String
    {
        return &mut self.value;
    }

    #[allow(dead_code)]
    pub fn set_value_copy(&mut self, value: &String)
    {
        self.value = value.clone();
    }

    #[allow(dead_code)]
    pub fn set_value_move(&mut self, value: String)
    {
        self.value = value;
    }
}

impl TokenTrait for TokenString
{
    fn get_type(&self) -> EnumTokenType
    {
        return EnumTokenType::STRING;
    }

    fn as_bool(&self) -> Result<bool, String>
    {
        return Err("TokenString is not a TokenBool".to_string());
    }

    fn as_char(&self) -> Result<char, String>
    {
        return Err("TokenString is not a TokenChar".to_string());
    }

    fn as_double(&self) -> Result<f64, String>
    {
        return Err("TokenString is not a TokenDouble".to_string());
    }

    fn as_string(&self) -> Result<&String, String>
    {
        return Ok(&self.get_value());
    }

    fn as_symbol(&self) -> Result<&String, String>
    {
        return Err("TokenString is not a TokenSymbol".to_string());
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
        false
    }

    fn is_null(&self) -> bool
    {
        false
    }

    fn is_string(&self) -> bool
    {
        true
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
    use crate::parser::token_string::TokenString;

    #[test]
    fn create_token_string()
    {
        let mut in_value = String::from("\"Hello, world!\"");
        let mut token = TokenString::new(in_value.clone());
        assert_eq!(token.get_type(), EnumTokenType::STRING);
        assert_eq!(token.get_value(), &in_value);

        in_value += " \"Hello, again, world!\"";
        token.set_value_copy(&in_value);
        assert_eq!(token.get_value(), &in_value);
    }
}



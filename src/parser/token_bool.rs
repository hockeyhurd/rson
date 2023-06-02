use crate::parser::token::{EnumTokenType, TokenTrait};

pub struct TokenBool
{
    value: bool,
}

impl TokenBool
{
    #[allow(dead_code)]
    pub fn new(value: bool) -> Self
    {
        Self { value }
    }

    #[allow(dead_code)]
    pub fn get_value(&self) -> bool
    {
        return self.value.into();
    }

    #[allow(dead_code)]
    pub fn set_value(&mut self, value: bool)
    {
        self.value = value.into();
    }
}

impl TokenTrait for TokenBool
{
    fn get_type(&self) -> EnumTokenType
    {
        return EnumTokenType::BOOL;
    }

    fn as_bool(&self) -> Result<bool, String>
    {
        return Ok(self.value);
    }

    fn as_double(&self) -> Result<f64, String>
    {
        return Err("TokenBool is not a TokenDouble".to_string());
    }

    fn as_string(&self) -> Result<&String, String>
    {
        return Err("TokenBool is not a TokenString".to_string());
    }

    fn as_symbol(&self) -> Result<&String, String>
    {
        return Err("TokenBool is not a TokenSymbol".to_string());
    }

    fn is_bool(&self) -> bool
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
    use crate::parser::token_bool::TokenBool;

    #[test]
    fn create_token_bool()
    {
        let mut in_value = true;
        let mut token = TokenBool::new(in_value);
        assert_eq!(token.get_type(), EnumTokenType::BOOL);
        assert_eq!(token.get_value(), in_value);

        in_value = false;
        token.set_value(in_value);
        assert_eq!(token.get_value(), in_value);
    }
}


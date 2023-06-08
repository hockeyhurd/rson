use crate::parser::lexer::Lexer;
use crate::rnodes::rnode::RNode;
use crate::rnodes::rnode_bool::RNodeBool;
use crate::rnodes::rnode_double::RNodeDouble;
use crate::rnodes::rnode_null::RNodeNull;
use crate::rnodes::rnode_string::RNodeString;

use super::token::{EnumTokenType, TokenTrait};

use std::rc::Rc;

pub struct Parser
{
    lexer: Lexer,
}

impl Parser
{
    #[allow(dead_code)]
    pub fn new(input: &String) -> Self
    {
        Self { lexer: Lexer::new_copy(&input) }
    }

    #[allow(dead_code)]
    pub fn new_move(input: String) -> Self
    {
        Self { lexer: Lexer::new_move(input) }
    }

    #[allow(dead_code)]
    pub fn from_file(path: &String) -> Self
    {
        let input = std::fs::read_to_string(&path).expect("Failed to read file");
        return Self::new_move(input);
    }

    #[allow(dead_code)]
    pub fn parse(&mut self) -> Result<Rc<dyn RNode>, String>
    {
        loop
        {
            let snapshot = self.lexer.snap();
            let token_result = self.lexer.next_token();

            match token_result
            {
                Ok(_token) =>
                {
                    // TODO: Continue pursuing this optimization at some point...
                    /*let mut opt_result = self.try_parse_symbol(Rc::clone(&token), "{");

                    if opt_result.is_some()
                    {
                        opt_result = self.try_parse_object();
                        // success = opt_result.is_some();
                    }*/

                    self.lexer.restore(&snapshot);
                    let opt_node_type = self.try_parse_type();

                    match opt_node_type
                    {
                        Some(node_type) => { return Ok(node_type); },
                        None => { return Err(String::from("Failed to parse token into an AST RNode")); },
                    }
                },
                Err(e) => { return Err(e); },
            }
        }

        // return Err(String::from("Un-expected error occurred. Is this a Parser bug?"));
    }

    /*fn try_parse_object(&mut self) -> Option<Rc<dyn TokenTrait>>
    {
        // TODO: Parse inner key, value pairs...
        let empty_string: String = String::from("");
        let token_result = self.lexer.next_token();

        match token_result
        {
            Ok(token) =>
            {
                if token.is_symbol() && token.as_symbol().unwrap_or(&empty_string) == "{"
                {
                    return Some(Rc::clone(&token));
                }

                return None;
            },
            Err(_) => { return None; },
        }

        return None;
    }*/

    #[allow(dead_code)]
    fn try_parse_symbol(&self, token: Rc<dyn TokenTrait>, expected_symbol: &str) -> Option<Rc<dyn TokenTrait>>
    {
        if token.is_symbol() && token.as_symbol().unwrap() == expected_symbol
        {
            return Some(token);
        }

        return None;
    }

    #[allow(dead_code)]
    fn try_parse_type(&mut self) -> Option<Rc<dyn RNode>>
    {
        let token_result = self.lexer.next_token();

        match token_result
        {
            Ok(token) =>
            {
                match token.get_type()
                {
                    EnumTokenType::BOOL => { return Some(Rc::new(RNodeBool::new(token.as_bool().unwrap()))); },
                    EnumTokenType::DOUBLE => { return Some(Rc::new(RNodeDouble::new(token.as_double().unwrap()))); },
                    EnumTokenType::NULL => { return Some(Rc::new(RNodeNull::new())); },
                    EnumTokenType::STRING => { return Some(Rc::new(RNodeString::new_copy(token.as_string().unwrap()))); },
                    _ => { panic!(); },
                }
            },
            Err(_) => { return None; },
        }
    }
}

#[cfg(test)]
mod tests
{
    #[allow(unused_imports)]
    use crate::parser::parser::Parser;
    use crate::rnodes::rnode::EnumNodeType;
    use crate::rnodes::rnode_bool::RNodeBool;
    use crate::rnodes::rnode_double::RNodeDouble;
    use crate::rnodes::rnode_null::RNodeNull;
    use crate::rnodes::rnode_string::RNodeString;

    #[test]
    fn parse_bool()
    {
        let input = String::from("true");
        let mut parser = Parser::new(&input);
        let node_type_result = parser.parse();

        assert!(node_type_result.is_ok());

        let rnode = node_type_result.unwrap();
        assert_eq!(rnode.get_node_type(), EnumNodeType::BOOL);

        let node_bool = rnode.downcast_rc::<RNodeBool>().map_err(|_| "Shouldn't happen").unwrap();
        assert!(node_bool.value);
    }

    #[test]
    fn parse_double()
    {
        let num: f64 = 123.456;
        let input = num.to_string();
        let mut parser = Parser::new(&input);
        let node_type_result = parser.parse();

        assert!(node_type_result.is_ok());

        let rnode = node_type_result.unwrap();
        assert_eq!(rnode.get_node_type(), EnumNodeType::DOUBLE);

        let node_double = rnode.downcast_rc::<RNodeDouble>().map_err(|_| "Shouldn't happen").unwrap();
        assert_eq!(node_double.value, num);
    }

    #[test]
    fn parse_null()
    {
        let input = String::from("null");
        let mut parser = Parser::new(&input);
        let node_type_result = parser.parse();

        assert!(node_type_result.is_ok());

        let rnode = node_type_result.unwrap();
        assert_eq!(rnode.get_node_type(), EnumNodeType::NULL);
    }

    #[test]
    fn parse_string()
    {
        let value = String::from("Hi");
        let input = String::from("\"Hi\"");
        let mut parser = Parser::new(&input);
        let node_type_result = parser.parse();

        assert!(node_type_result.is_ok());

        let rnode = node_type_result.unwrap();
        assert_eq!(rnode.get_node_type(), EnumNodeType::STRING);

        let node_string = rnode.downcast_rc::<RNodeString>().map_err(|_| "Shouldn't happen").unwrap();
        assert_eq!(node_string.get_value(), &value);
    }
}


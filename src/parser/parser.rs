use crate::parser::lexer::Lexer;
use crate::rnodes::rnode::RNode;
use crate::rnodes::rnode_array::RNodeArray;
use crate::rnodes::rnode_bool::RNodeBool;
use crate::rnodes::rnode_double::RNodeDouble;
use crate::rnodes::rnode_null::RNodeNull;
use crate::rnodes::rnode_object::RNodeObject;
use crate::rnodes::rnode_string::RNodeString;

use super::token::{EnumTokenType, TokenTrait};

use std::rc::Rc;
use std::collections::{BTreeMap, HashMap};

pub struct Parser
{
    lexer: Lexer,
    guess_table: HashMap<String, fn(&mut Parser, Rc<dyn TokenTrait>) -> Option<Rc<dyn RNode>>>,
}

impl Parser
{
    #[allow(dead_code)]
    pub fn new_copy(input: &String) -> Self
    {
        let mut result = Self { lexer: Lexer::new_copy(&input), guess_table: HashMap::new() };
        result.init_guess_table();
        return result;
    }

    #[allow(dead_code)]
    pub fn new_move(input: String) -> Self
    {
        let mut result = Self { lexer: Lexer::new_move(input), guess_table: HashMap::new() };
        result.init_guess_table();
        return result;
    }

    fn init_guess_table(&mut self)
    {
        self.guess_table.insert("[".to_string(), try_parse_array);
        self.guess_table.insert("{".to_string(), try_parse_object);
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
                    EnumTokenType::SYMBOL =>
                    {
                        let symbol = token.as_symbol().unwrap();
                        let opt_guess = &self.guess_table.get(symbol);

                        if opt_guess.is_some()
                        {
                            let fp = opt_guess.expect("It's a tarp!!");
                            return fp(self, Rc::clone(&token));
                        }

                        return None;
                    },
                    _ => { panic!(); },
                }
            },
            Err(_) => { return None; },
        }
    }
}

fn try_parse_array(parser: &mut Parser, token_in: Rc<dyn TokenTrait>) -> Option<Rc<dyn RNode>>
{
    if !token_in.is_symbol() || token_in.as_symbol().unwrap() != "["
    {
        return None;
    }

    let mut last_was_comma = false;
    let mut nodes = Vec::<Rc<dyn RNode>>::new();

    loop
    {
        let snapshot = parser.lexer.snap();
        let peek_token_result = parser.lexer.next_token();

        match peek_token_result
        {
            Ok(peek_token) =>
            {
                // If the last token was a comma, expect a type next or invalid
                if last_was_comma
                {
                    last_was_comma = false;

                    // Restore since try_parse_type will expect the start of the next Token we want
                    // to try and parse next.
                    parser.lexer.restore(&snapshot);
                    let opt_type_node = parser.try_parse_type();

                    match opt_type_node
                    {
                        Some(type_node) => { nodes.push(type_node); },
                        None => { /* TODO: log error?? */ return None; },
                    }
                }

                else if peek_token.is_symbol()
                {
                    let symbol = peek_token.as_symbol().unwrap();

                    // Check for end of array
                    if symbol == "]"
                    {
                        break;
                    }

                    // See if it's a comma then
                    else if symbol == ","
                    {
                        last_was_comma = true;
                    }

                    // Is a sub-array??
                    else if symbol == "["
                    {
                        let opt_array_type = try_parse_array(parser, Rc::clone(&peek_token));

                        match opt_array_type
                        {
                            Some(array_type) => { nodes.push(array_type); },
                            None => { return None; },
                        }
                    }

                    else
                    {
                        // TODO: print error??
                        return None;
                    }
                }

                else if peek_token.is_bool()
                {
                    last_was_comma = false;
                    nodes.push(Rc::new(RNodeBool::new(peek_token.as_bool().unwrap())));
                }

                else if peek_token.is_double()
                {
                    last_was_comma = false;
                    nodes.push(Rc::new(RNodeDouble::new(peek_token.as_double().unwrap())));
                }

                else if peek_token.is_null()
                {
                    last_was_comma = false;
                    nodes.push(Rc::new(RNodeNull::new()));
                }

                // TODO: Add more token types here...
                else if peek_token.is_string()
                {
                    last_was_comma = false;
                    nodes.push(Rc::new(RNodeString::new_copy(peek_token.as_string().unwrap())));
                }

                // TODO: Can we reduce logic with the above 'else' statement somehow??
                else
                {
                    // TODO: print error??
                    return None;
                }
            },
            Err(_) => { return None; }
        }
    }

    return Some(Rc::new(RNodeArray::new(nodes)));
}

fn try_parse_object(parser: &mut Parser, token_in: Rc<dyn TokenTrait>) -> Option<Rc<dyn RNode>>
{
    if !token_in.is_symbol() || token_in.as_symbol().unwrap() != "{"
    {
        return None;
    }

    let mut last_was_comma: bool = true;
    let mut nodes = BTreeMap::<String, Rc<dyn RNode>>::new();

    loop
    {
        let snapshot = parser.lexer.snap();
        let mut opt_peek_token = parser.lexer.next_token();

        // First expect the String key
        match opt_peek_token
        {
            Ok(mut peek_token) =>
            {
                if !last_was_comma
                {
                    last_was_comma = true;

                    if !peek_token.is_symbol()
                    {
                        println!("Error: expected a ','");
                        return None;
                    }

                    let symbol = peek_token.as_symbol().unwrap();

                    if symbol == "}"
                    {
                        break;
                    }

                    else if symbol != ","
                    {
                        println!("Error: expected a ','");
                        return None;
                    }
                }

                else
                {
                    last_was_comma = false;

                    if !peek_token.is_string()
                    {
                        if peek_token.is_symbol() && peek_token.as_symbol().unwrap() == "}"
                        {
                            break;
                        }

                        // TODO: Log the error here??
                        return None;
                    }

                    let key = peek_token.as_string().unwrap().clone();

                    // Next expect a ':'
                    opt_peek_token = parser.lexer.next_token();

                    if opt_peek_token.is_err()
                    {
                        return None;
                    }

                    peek_token = opt_peek_token.unwrap();

                    if !peek_token.is_symbol() || peek_token.as_symbol().unwrap() != ":"
                    {
                        return None;
                    }

                    // Last should be a value of some RNode type.
                    let opt_node_type = parser.try_parse_type();

                    match opt_node_type
                    {
                        Some(node_type) =>
                        {
                            nodes.insert(key, Rc::clone(&node_type));
                        },
                        None => { println!("Error: expected to parse an RNode type at {0}", snapshot.to_string()); return None; },
                    }
                }
            },
            Err(_) => { return None; },
        }
    }

    return Some(Rc::new(RNodeObject::new(nodes)));
}

#[cfg(test)]
mod tests
{
    use crate::parser::parser::Parser;
    use crate::rnodes::rnode::EnumNodeType;
    use crate::rnodes::rnode_array::RNodeArray;
    use crate::rnodes::rnode_bool::RNodeBool;
    use crate::rnodes::rnode_double::RNodeDouble;

    use crate::rnodes::rnode::RNode;
    use crate::rnodes::rnode_null::RNodeNull;
    use crate::rnodes::rnode_object::RNodeObject;
    use crate::rnodes::rnode_string::RNodeString;

    #[test]
    fn parse_empty_array()
    {
        let input = String::from("[]");
        let mut parser = Parser::new_copy(&input);
        let node_type_result = parser.parse();

        assert!(node_type_result.is_ok());

        let rnode = node_type_result.unwrap();
        assert_eq!(rnode.get_node_type(), EnumNodeType::ARRAY);

        let node_array = rnode.downcast_rc::<RNodeArray>().map_err(|_| "Shouldn't happen").unwrap();
        assert!(node_array.is_empty());
    }

    #[test]
    fn parse_empty_array_with_space()
    {
        let input = String::from("[   ]");
        let mut parser = Parser::new_copy(&input);
        let node_type_result = parser.parse();

        assert!(node_type_result.is_ok());

        let rnode = node_type_result.unwrap();
        assert_eq!(rnode.get_node_type(), EnumNodeType::ARRAY);

        let node_array = rnode.downcast_rc::<RNodeArray>().map_err(|_| "Shouldn't happen").unwrap();
        assert!(node_array.is_empty());
    }

    #[test]
    fn parse_array_with_inner_array()
    {
        let input = String::from("[ [] ]");
        let mut parser = Parser::new_copy(&input);
        let node_type_result = parser.parse();

        assert!(node_type_result.is_ok());

        let rnode = node_type_result.unwrap();
        assert_eq!(rnode.get_node_type(), EnumNodeType::ARRAY);

        let node_array = rnode.downcast_rc::<RNodeArray>().map_err(|_| "Shouldn't happen").unwrap();
        assert!(!node_array.is_empty());
        assert_eq!(node_array.len(), 1);

        let opt_node_array = node_array.get(0);
        assert!(opt_node_array.is_some());

        let node_array2 = opt_node_array.unwrap().downcast_rc::<RNodeArray>().map_err(|_| "Shouldn't happen").unwrap();
        assert!(node_array2.is_empty());
        assert_eq!(node_array2.len(), 0);
    }

    #[test]
    fn parse_array_with_inner_bool()
    {
        let input = String::from("[ true ]");
        let mut parser = Parser::new_copy(&input);
        let node_type_result = parser.parse();

        assert!(node_type_result.is_ok());

        let rnode = node_type_result.unwrap();
        assert_eq!(rnode.get_node_type(), EnumNodeType::ARRAY);

        let node_array = rnode.downcast_rc::<RNodeArray>().map_err(|_| "Shouldn't happen").unwrap();
        assert!(!node_array.is_empty());
        assert_eq!(node_array.len(), 1);

        let opt_node_bool = node_array.get(0);
        assert!(opt_node_bool.is_some());

        let node_bool = opt_node_bool.unwrap().downcast_rc::<RNodeBool>().map_err(|_| "Shouldn't happen").unwrap();
        assert!(node_bool.value);
    }

    #[test]
    fn parse_array_with_inner_double()
    {
        let input = String::from("[ 123.456 ]");
        let mut parser = Parser::new_copy(&input);
        let node_type_result = parser.parse();

        assert!(node_type_result.is_ok());

        let rnode = node_type_result.unwrap();
        assert_eq!(rnode.get_node_type(), EnumNodeType::ARRAY);

        let node_array = rnode.downcast_rc::<RNodeArray>().map_err(|_| "Shouldn't happen").unwrap();
        assert!(!node_array.is_empty());
        assert_eq!(node_array.len(), 1);

        let opt_node_double = node_array.get(0);
        assert!(opt_node_double.is_some());

        let node_double = opt_node_double.unwrap().downcast_rc::<RNodeDouble>().map_err(|_| "Shouldn't happen").unwrap();
        assert_eq!(node_double.value, 123.456);
    }

    #[test]
    fn parse_array_with_inner_null()
    {
        let input = String::from("[ null ]");
        let mut parser = Parser::new_copy(&input);
        let node_type_result = parser.parse();

        assert!(node_type_result.is_ok());

        let rnode = node_type_result.unwrap();
        assert_eq!(rnode.get_node_type(), EnumNodeType::ARRAY);

        let node_array = rnode.downcast_rc::<RNodeArray>().map_err(|_| "Shouldn't happen").unwrap();
        assert!(!node_array.is_empty());
        assert_eq!(node_array.len(), 1);

        let opt_node_null = node_array.get(0);
        assert!(opt_node_null.is_some());
    }

    #[test]
    fn parse_array_with_inner_string()
    {
        let value = String::from("Hello, world!");
        let input = String::from("[ \"Hello, world!\" ]");
        let mut parser = Parser::new_copy(&input);
        let node_type_result = parser.parse();

        assert!(node_type_result.is_ok());

        let rnode = node_type_result.unwrap();
        assert_eq!(rnode.get_node_type(), EnumNodeType::ARRAY);

        let node_array = rnode.downcast_rc::<RNodeArray>().map_err(|_| "Shouldn't happen").unwrap();
        assert!(!node_array.is_empty());
        assert_eq!(node_array.len(), 1);

        let opt_node_string = node_array.get(0);
        assert!(opt_node_string.is_some());

        let node_string = opt_node_string.unwrap().downcast_rc::<RNodeString>().map_err(|_| "Shouldn't happen").unwrap();
        assert_eq!(node_string.get_value(), &value);
    }

    #[test]
    fn parse_array_with_inner_bool_and_double()
    {
        let input = String::from("[ true, 123.456 ]");
        let mut parser = Parser::new_copy(&input);
        let node_type_result = parser.parse();

        assert!(node_type_result.is_ok());

        let rnode = node_type_result.unwrap();
        assert_eq!(rnode.get_node_type(), EnumNodeType::ARRAY);

        let node_array = rnode.downcast_rc::<RNodeArray>().map_err(|_| "Shouldn't happen").unwrap();
        assert!(!node_array.is_empty());
        assert_eq!(node_array.len(), 2);

        let opt_node_bool = node_array.get(0);
        assert!(opt_node_bool.is_some());

        let node_bool = opt_node_bool.unwrap().downcast_rc::<RNodeBool>().map_err(|_| "Shouldn't happen").unwrap();
        assert!(node_bool.value);

        let opt_node_double = node_array.get(1);
        assert!(opt_node_double.is_some());

        let node_double = opt_node_double.unwrap().downcast_rc::<RNodeDouble>().map_err(|_| "Shouldn't happen").unwrap();
        assert_eq!(node_double.value, 123.456);
    }

    #[test]
    fn parse_array_with_inner_all_nodes()
    {
        let input = String::from("[ [ ], true, 123.456, null, \"Hello\" ]");
        let mut parser = Parser::new_copy(&input);
        let node_type_result = parser.parse();

        assert!(node_type_result.is_ok());

        let rnode = node_type_result.unwrap();
        assert_eq!(rnode.get_node_type(), EnumNodeType::ARRAY);

        let node_array = rnode.downcast_rc::<RNodeArray>().map_err(|_| "Shouldn't happen").unwrap();
        assert!(!node_array.is_empty());
        assert_eq!(node_array.len(), 5);

        let opt_sub_array = node_array.get(0);
        assert!(opt_sub_array.is_some());

        let node_sub_array = opt_sub_array.unwrap().downcast_rc::<RNodeArray>().map_err(|_| "Shouldn't happen").unwrap();
        assert!(node_sub_array.is_empty());
        assert_eq!(node_sub_array.len(), 0);

        let opt_node_bool = node_array.get(1);
        assert!(opt_node_bool.is_some());

        let node_bool = opt_node_bool.unwrap().downcast_rc::<RNodeBool>().map_err(|_| "Shouldn't happen").unwrap();
        assert!(node_bool.value);

        let opt_node_double = node_array.get(2);
        assert!(opt_node_double.is_some());

        let node_double = opt_node_double.unwrap().downcast_rc::<RNodeDouble>().map_err(|_| "Shouldn't happen").unwrap();
        assert_eq!(node_double.value, 123.456);

        let opt_node_null = node_array.get(3);
        assert!(opt_node_null.is_some());

        let opt_node_string = node_array.get(4);
        assert!(opt_node_string.is_some());

        let node_string = opt_node_string.unwrap().downcast_rc::<RNodeString>().map_err(|_| "Shouldn't happen").unwrap();
        assert_eq!(node_string.get_value(), "Hello");
    }


    #[test]
    fn parse_bool()
    {
        let input = String::from("true");
        let mut parser = Parser::new_copy(&input);
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
        let mut parser = Parser::new_copy(&input);
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
        let mut parser = Parser::new_copy(&input);
        let node_type_result = parser.parse();

        assert!(node_type_result.is_ok());

        let rnode = node_type_result.unwrap();
        assert_eq!(rnode.get_node_type(), EnumNodeType::NULL);
    }

    #[test]
    fn parse_object_single_pair()
    {
        let key = String::from("key");
        let input = String::from("{ \"key\": 123.456 }");
        let mut parser = Parser::new_copy(&input);
        let node_type_result = parser.parse();

        assert!(node_type_result.is_ok());

        let rnode = node_type_result.unwrap();
        assert_eq!(rnode.get_node_type(), EnumNodeType::OBJECT);

        let node_object = rnode.downcast_rc::<RNodeObject>().map_err(|_| "Shouldn't happen").unwrap();
        assert!(!node_object.is_empty());
        assert_eq!(node_object.len(), 1);

        let opt_rnode_value = node_object.get(&key);
        assert!(opt_rnode_value.is_some());

        let rnode_value = opt_rnode_value.unwrap().downcast_rc::<RNodeDouble>().map_err(|_| "Shouldn't happen").unwrap();
        assert_eq!(rnode_value.value, 123.456);
    }

    #[test]
    fn parse_object_single_pair_with_empty_string()
    {
        let key = String::from("key");
        let input = String::from("{ \"key\": \"\" }");
        let mut parser = Parser::new_copy(&input);
        let node_type_result = parser.parse();

        assert!(node_type_result.is_ok());

        let rnode = node_type_result.unwrap();
        assert_eq!(rnode.get_node_type(), EnumNodeType::OBJECT);

        let node_object = rnode.downcast_rc::<RNodeObject>().map_err(|_| "Shouldn't happen").unwrap();
        assert!(!node_object.is_empty());
        assert_eq!(node_object.len(), 1);

        let opt_rnode_value = node_object.get(&key);
        assert!(opt_rnode_value.is_some());

        let rnode_value = opt_rnode_value.unwrap().downcast_rc::<RNodeString>().map_err(|_| "Shouldn't happen").unwrap();
        assert_eq!(rnode_value.get_value(), "");
    }

    #[test]
    fn parse_object_double_pair()
    {
        let key0 = String::from("key0");
        let key1 = String::from("key1");
        let input = String::from("{ \"key0\": 123.456, \"key1\": -42 }");
        let mut parser = Parser::new_copy(&input);
        let node_type_result = parser.parse();

        assert!(node_type_result.is_ok());

        let rnode = node_type_result.unwrap();
        assert_eq!(rnode.get_node_type(), EnumNodeType::OBJECT);

        let node_object = rnode.downcast_rc::<RNodeObject>().map_err(|_| "Shouldn't happen").unwrap();
        assert!(!node_object.is_empty());
        assert_eq!(node_object.len(), 2);

        let opt_rnode_value = node_object.get(&key0);
        assert!(opt_rnode_value.is_some());

        let rnode_value = opt_rnode_value.unwrap().downcast_rc::<RNodeDouble>().map_err(|_| "Shouldn't happen").unwrap();
        assert_eq!(rnode_value.value, 123.456);

        let opt_rnode_value2 = node_object.get(&key1);
        assert!(opt_rnode_value2.is_some());

        let rnode_value2 = opt_rnode_value2.unwrap().downcast_rc::<RNodeDouble>().map_err(|_| "Shouldn't happen").unwrap();
        assert_eq!(rnode_value2.value, -42.0);
    }

    #[test]
    fn parse_object_five_pair()
    {
        let key0 = String::from("key0");
        let key1 = String::from("key1");
        let key2 = String::from("key2");
        let key3 = String::from("key3");
        let key4 = String::from("key4");
        let inner_key1 = String::from("1");
        let inner_key2 = String::from("nil");
        let input = String::from("{ \"key0\": 123.456, \"key1\": -42, \"key2\": false, \"key3\": \"Hello\", \"key4\": { \"1\": 1, \"nil\": null } }");
        let mut parser = Parser::new_copy(&input);
        let node_type_result = parser.parse();

        assert!(node_type_result.is_ok());

        let rnode = node_type_result.unwrap();
        assert_eq!(rnode.get_node_type(), EnumNodeType::OBJECT);

        let node_object = rnode.downcast_rc::<RNodeObject>().map_err(|_| "Shouldn't happen").unwrap();
        assert!(!node_object.is_empty());
        assert_eq!(node_object.len(), 5);

        let opt_rnode_value = node_object.get(&key0);
        assert!(opt_rnode_value.is_some());

        let rnode_value = opt_rnode_value.unwrap().downcast_rc::<RNodeDouble>().map_err(|_| "Shouldn't happen").unwrap();
        assert_eq!(rnode_value.value, 123.456);

        let opt_rnode_value1 = node_object.get(&key1);
        assert!(opt_rnode_value1.is_some());

        let rnode_value1 = opt_rnode_value1.unwrap().downcast_rc::<RNodeDouble>().map_err(|_| "Shouldn't happen").unwrap();
        assert_eq!(rnode_value1.value, -42.0);

        let opt_rnode_value2 = node_object.get(&key2);
        assert!(opt_rnode_value2.is_some());

        let rnode_value2 = opt_rnode_value2.unwrap().downcast_rc::<RNodeBool>().map_err(|_| "Shouldn't happen").unwrap();
        assert!(!rnode_value2.value);

        let opt_rnode_value3 = node_object.get(&key3);
        assert!(opt_rnode_value3.is_some());

        let rnode_value3 = opt_rnode_value3.unwrap().downcast_rc::<RNodeString>().map_err(|_| "Shouldn't happen").unwrap();
        assert_eq!(rnode_value3.get_value(), "Hello");

        let opt_rnode_value4 = node_object.get(&key4);
        assert!(opt_rnode_value4.is_some());

        let rnode_value4 = opt_rnode_value4.unwrap().downcast_rc::<RNodeObject>().map_err(|_| "Shouldn't happen").unwrap();
        assert!(!rnode_value4.is_empty());
        assert_eq!(rnode_value4.len(), 2);

        let opt_inner_double = rnode_value4.get(&inner_key1);
        assert!(opt_inner_double.is_some());

        let rnode_inner_double = opt_inner_double.unwrap().downcast_rc::<RNodeDouble>().map_err(|_| "Shouldn't happen").unwrap();
        assert_eq!(rnode_inner_double.value, 1.0);

        let opt_inner_nil = rnode_value4.get(&inner_key2);
        assert!(opt_inner_nil.is_some());

        let rnode_inner_null = opt_inner_nil.unwrap().downcast_rc::<RNodeNull>().map_err(|_| "Shouldn't happen").unwrap();
        assert_eq!(rnode_inner_null.get_node_type(), EnumNodeType::NULL);
    }

    #[test]
    fn parse_string()
    {
        let value = String::from("Hi");
        let input = String::from("\"Hi\"");
        let mut parser = Parser::new_copy(&input);
        let node_type_result = parser.parse();

        assert!(node_type_result.is_ok());

        let rnode = node_type_result.unwrap();
        assert_eq!(rnode.get_node_type(), EnumNodeType::STRING);

        let node_string = rnode.downcast_rc::<RNodeString>().map_err(|_| "Shouldn't happen").unwrap();
        assert_eq!(node_string.get_value(), &value);
    }
}


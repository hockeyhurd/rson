#[allow(unused_imports)]
use crate::parser::snapshot::Snapshot;
use crate::parser::token::TokenTrait;
use crate::parser::token_bool::TokenBool;
use crate::parser::token_char::TokenChar;
use crate::parser::token_double::TokenDouble;
use crate::parser::token_string::TokenString;
use crate::parser::token_symbol::TokenSymbol;
use crate::utils::string_utils::StringBuilder;

use std::collections::hash_map::HashMap;
use std::rc::Rc;

use super::token_null::TokenNull;

pub struct Lexer
{
    input: String,
    index: usize,
    lookup_table: HashMap<char, fn(&mut Lexer, char) -> Result<Rc<dyn TokenTrait>, String>>,
    escape_char_table: HashMap<char, char>,
    buffer: StringBuilder,
}

impl Lexer
{
    pub fn new_copy(input: &String) -> Self
    {
        let mut result = Self
        {
            input: input.clone(), index: 0,
            lookup_table: HashMap::new(), escape_char_table: HashMap::new(),
            buffer: StringBuilder::new(4096)
        };

        result.init_table();

        return result;
    }

    pub fn new_move(input: String) -> Self
    {
        let mut result = Self
        {
            input: input, index: 0,
            lookup_table: HashMap::new(), escape_char_table: HashMap::new(),
            buffer: StringBuilder::new(4096)
        };

        result.init_table();

        return result;
    }

    fn init_table(&mut self)
    {
        self.lookup_table.insert('\\', handle_leading_escape);
        self.lookup_table.insert('.', handle_number);
        self.lookup_table.insert('-', handle_number);
        self.lookup_table.insert('"', handle_string);
        self.lookup_table.insert(',', handle_single_char_symbol);
        self.lookup_table.insert(':', handle_single_char_symbol);
        self.lookup_table.insert('{', handle_single_char_symbol);
        self.lookup_table.insert('}', handle_single_char_symbol);
        self.lookup_table.insert('[', handle_single_char_symbol);
        self.lookup_table.insert(']', handle_single_char_symbol);

        for ch in 'A'..'Z'
        {
            self.lookup_table.insert(ch, handle_symbol);
        }

        for ch in 'a'..'z'
        {
            self.lookup_table.insert(ch, handle_symbol);
        }

        for i in 0..10u32
        {
            self.lookup_table.insert(char::from_digit(i, 10).expect("Failed to convert 'i' to a char"), handle_number);
        }

        self.escape_char_table.insert('"', '\"');
        self.escape_char_table.insert('\'', '\'');
        self.escape_char_table.insert('\\', '\\');
        self.escape_char_table.insert('n', '\n');
        self.escape_char_table.insert('r', '\r');
    }

    #[allow(dead_code)]
    pub fn get_input(&self) -> &String
    {
        return &self.input;
    }

    #[allow(dead_code)]
    fn get_position(&self) -> usize
    {
        return self.index;
    }

    pub fn next_token(&mut self) -> Result<Rc<dyn TokenTrait>, String>
    {
        if self.index < self.input.len()
        {
            return self.next_token_internal();
        }

        return Err("Out of tokens".to_string());
    }

    #[allow(dead_code)]
    pub fn restore(&mut self, snapshot: &Snapshot)
    {
        self.index = snapshot.get_start_pos();
    }

    #[allow(dead_code)]
    pub fn snap(&self) -> Snapshot
    {
        return Snapshot::new(self.index);
    }

    fn next_token_internal(&mut self) -> Result<Rc<dyn TokenTrait>, String>
    {
        let _ = self.skip_whitespace();
        let ch = self.next_char();

        match ch
        {
            Some(the_char) =>
            {
                // Clear the buffer to make sure there isn't any junk in it.
                // We do this here to the handler functions do not need to worry.
                self.buffer.clear();

                // println!("{0}", the_char);
                let lookup_func: Option<&fn(&mut Lexer, char) -> Result<Rc<dyn TokenTrait>, String>> = self.lookup_table.get(&the_char);

                match lookup_func
                {
                    Some(func) =>
                    {
                        let result = func(self, the_char);
                        return result;
                    },
                    None => { return Err(String::from("Error")); },
                }
            },
            None => { return Err(String::from("End of input (EOF)")); },
        }
    }

    fn next_char(&mut self) -> Option<char>
    {
        let chars = &self.input.as_bytes();

        if self.index < chars.len()
        {
            let ch = chars[self.index];
            self.index += 1;
            return Some(ch as char);
        }

        return None;
    }

    fn put_back(&mut self, delta: usize)
    {
        if delta > self.index
        {
            self.index = 0;
        }

        else
        {
            self.index -= delta;
        }
    }

    fn skip_whitespace(&mut self) -> usize
    {
        // TODO: Don't use as_bytes because of UTF
        let mut delta: usize = self.index;
        let chars = &self.input.as_bytes();
        let mut ch;

        loop
        {
            if self.index >= chars.len()
            {
                return 0;
            }

            ch = chars[self.index];

            if !ch.is_ascii_whitespace()
            {
                break;
            }

            self.index += 1;
        }

        if self.index >= chars.len()
        {
            return 0;
        }

        // Note: delta is already set to the start position, so we reuse it here
        // for this calculation.
        delta = self.index - delta;
        return delta;
    }
}

fn handle_leading_escape(inst: &mut Lexer, _ch: char) -> Result<Rc<dyn TokenTrait>, String>
{
    let lookahead_opt = inst.next_char();

    match lookahead_opt
    {
        Some(lookahead) =>
        {
            let opt_escape_char = inst.escape_char_table.get(&lookahead);

            match opt_escape_char
            {
                Some(escape_char) => { return Ok(Rc::new(TokenChar::new(*escape_char))); },
                None => { return Err(String::from("Error: Not a supported escape sequence")); },
            }
        },
        None => { return Err(String::from("Error: failed lookahead")); },
    }
}

fn handle_number(inst: &mut Lexer, ch: char) -> Result<Rc<dyn TokenTrait>, String>
{
    inst.buffer.append_char(ch);
    let mut seen_dot = false;
    let mut seen_e = false;
    let mut seen_neg = false;
    let mut seen_number = ch >= '0' && ch <= '9';

    loop
    {
        let cur_char_opt = inst.next_char();

        match cur_char_opt
        {
            Some(cur_char) =>
            {
                if cur_char >= '0' && cur_char <= '9'
                {
                    seen_number = true;
                    inst.buffer.append_char(cur_char);
                }

                else if cur_char == '.'
                {
                    if seen_dot
                    {
                        return Err(String::from("Error parsing number where a second '.' was discovered"));
                    }

                    seen_dot = true;
                    inst.buffer.append_char(cur_char);
                }

                else if cur_char == 'e' || cur_char == 'E'
                {
                    if seen_e
                    {
                        return Err(String::from("Error parsing number where a second 'e' was discovered"));
                    }

                    seen_e = true;
                    inst.buffer.append_char(cur_char);
                }

                else if cur_char == '-'
                {
                    if !seen_e
                    {
                        return Err(String::from("Error parsing number where a '-' was discovered before 'e'"));
                    }

                    else if seen_neg
                    {
                        return Err(String::from("Error parsing number where a second '-' was discovered"));
                    }

                    seen_neg = true;
                    inst.buffer.append_char(cur_char);
                }

                else
                {
                    inst.put_back(1);
                    break;
                }
            },
            None => { /*println!("Logic error in handle_symbol while getting the next char");*/ break; },
        }
    }

    if inst.buffer.empty()
    {
        return Err(String::from("Logic error in handling symbol"));
    }

    else if !seen_number
    {
        // TODO: print out said input would be helpful...
        return Err(String::from("Did not see a number while lexing input"));
    }

    let output = inst.buffer.to_string();
    // println!("debug output: {0}, {1}", output, output.len());

    let opt_num = to_num(output);

    if opt_num.is_some()
    {
        return Ok(Rc::new(TokenDouble::new(opt_num.unwrap())));
    }

    return Err(String::from("Error parsing number"));
}

fn handle_string(inst: &mut Lexer, _ch: char) -> Result<Rc<dyn TokenTrait>, String>
{
    // We don't add it to the buffer because we want to remove it and just get
    // the raw value.
    // inst.buffer.append_char(ch);
    let mut last_was_escape = false;
    let mut saw_close_quote = false;

    loop
    {
        let cur_char_opt = inst.next_char();

        match cur_char_opt
        {
            Some(cur_char) =>
            {
                // Handle escapes
                if cur_char == '\\'
                {
                    if last_was_escape
                    {
                        let opt_escape_char = inst.escape_char_table.get(&cur_char);

                        match opt_escape_char
                        {
                            Some(escape_char) => { inst.buffer.append_char(*escape_char); },
                            None => { return Err(String::from("Error not a supported escape character")); },
                        }

                        last_was_escape = false;
                    }

                    else
                    {
                        last_was_escape = true;
                    }
                }

                else if last_was_escape
                {
                    let opt_escape_char = inst.escape_char_table.get(&cur_char);

                    match opt_escape_char
                    {
                        Some(escape_char) => { inst.buffer.append_char(*escape_char); },
                        None => { return Err(String::from("Error not a supported escape character")); },
                    }

                    last_was_escape = false;
                }

                else if cur_char == '"'
                {
                    // This marks the end of the string.
                    if !last_was_escape
                    {
                        saw_close_quote = true;
                        break;
                    }

                    last_was_escape = false;
                    inst.buffer.append_char(cur_char);
                }

                // Normal char
                else
                {
                    inst.buffer.append_char(cur_char);
                }
            },
            None => { /*println!("Logic error in handle_string while getting the next char");*/ break; },
        }
    }

    if !saw_close_quote
    {
        return Err(String::from("Error: Missing closing double-quote ('\"')."));
    }

    let output = inst.buffer.to_string();
    // println!("debug output: {0}, {1}", output, output.len());

    return Ok(Rc::new(TokenString::new(output)));
}

fn handle_symbol(inst: &mut Lexer, ch: char) -> Result<Rc<dyn TokenTrait>, String>
{
    inst.buffer.append_char(ch);

    loop
    {
        let cur_char_opt = inst.next_char();

        match cur_char_opt
        {
            Some(cur_char) =>
            {
                if !cur_char.is_ascii_whitespace() && (cur_char.is_digit(10) || cur_char.is_alphabetic())
                {
                    inst.buffer.append_char(cur_char);
                }

                else
                {
                    inst.put_back(1);
                    break;
                }
            },
            None => { /*println!("Logic error in handle_symbol while getting the next char");*/ break; },
        }
    }

    if inst.buffer.empty()
    {
        return Err(String::from("Logic error in handling symbol"));
    }

    let output = inst.buffer.to_string();
    // println!("debug output: {0}, {1}", output, output.len());

    // Check to see if this symbol was true or false, thus making it a TokenBool.
    if output == "true"
    {
        return Ok(Rc::new(TokenBool::new(true)));
    }

    else if output == "false"
    {
        return Ok(Rc::new(TokenBool::new(false)));
    }

    else if output == "null"
    {
        return Ok(Rc::new(TokenNull::new()));
    }

    return Ok(Rc::new(TokenSymbol::new(output)));
}

fn handle_single_char_symbol(_inst: &mut Lexer, ch: char) -> Result<Rc<dyn TokenTrait>, String>
{
    return Ok(Rc::new(TokenSymbol::new(ch.to_string())));
}

fn to_num(input: String) -> Option<f64>
{
    if input.len() == 0
    {
        return None;
    }

    let mut result = 0.0;
    let mut multicand = 10.0;
    let mut index: usize = 0;
    let mut seen_dot = false;
    let is_neg = input.as_bytes()[index] == ('-' as u8);

    // Need to offset if negative.
    if is_neg
    {
        index += 1;
    }

    for ch in input.chars().skip(is_neg as usize)
    {
        if ch >= '0' && ch <= '9'
        {
            let as_digit = (ch as u32) - ('0' as u32);
            // println!("[DEBUG]: digit is {0}", as_digit);

            if !seen_dot
            {
                result *= multicand;
                result += as_digit as f64;
            }

            else
            {
                result += (as_digit as f64) * multicand;
            }
        }

        else if ch == '.'
        {
            multicand = 0.1;
            seen_dot = true;
            index += 1;
            continue;
        }

        else if ch == 'e' || ch == 'E'
        {
            index += 1;
            let exp_result = to_num(String::from(input.get(index..).unwrap()));

            if exp_result.is_none()
            {
                return None;
            }

            const EXP: f64 = 10.0;
            result *= EXP.powf(exp_result.unwrap());

            break;
        }

        if seen_dot
        {
            multicand /= 10.0;
        }

        index += 1;
        // println!("[DEBUG]: Current state result is {0}, multicand is {1}", result, multicand);
    }

    if is_neg
    {
        result = -result;
    }

    // println!("[DEBUG]: Result is {0}", result);
    return Some(result);
}

#[cfg(test)]
mod tests
{
    #[allow(unused_imports)]
    use crate::parser::token::{EnumTokenType, TokenTrait};
    use crate::parser::lexer::Lexer;

    #[test]
    fn lex_accepts_empty_input()
    {
        let input = String::from("");
        let mut lexer = Lexer::new_copy(&input);

        let token_result = lexer.next_token();
        assert!(token_result.is_err());
    }

    #[test]
    fn lex_accepts_curly_brackets()
    {
        let input = String::from(" { } ");
        let first_token = String::from("{");
        let second_token = String::from("}");
        let mut lexer = Lexer::new_copy(&input);

        let mut token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &first_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &second_token);
        }

        token_result = lexer.next_token();
        assert!(token_result.is_err());
    }

    #[test]
    fn lex_token_bool_true()
    {
        let first_token = String::from("{");
        let second_token = true;
        let third_token = String::from("}");
        let input = String::from("{ true }");
        let mut lexer = Lexer::new_copy(&input);

        let mut token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &first_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::BOOL);
            assert!(token.is_bool());
            assert_eq!(token.as_bool().unwrap(), second_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &third_token);
        }

        token_result = lexer.next_token();
        assert!(token_result.is_err());
    }

    #[test]
    fn lex_token_bool_false()
    {
        let first_token = String::from("{");
        let second_token = false;
        let third_token = String::from("}");
        let input = String::from("{ false }");
        let mut lexer = Lexer::new_copy(&input);

        let mut token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &first_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::BOOL);
            assert!(token.is_bool());
            assert_eq!(token.as_bool().unwrap(), second_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &third_token);
        }
    }

    #[test]
    fn lex_token_double()
    {
        let first_token = String::from("{");
        let second_token = 123.45;
        let third_token = String::from("}");
        let input = String::from("{ 123.45 }");
        let mut lexer = Lexer::new_copy(&input);

        let mut token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &first_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::DOUBLE);
            assert!(token.is_double());
            assert_eq!(token.as_double().unwrap(), second_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &third_token);
        }

        token_result = lexer.next_token();
        assert!(token_result.is_err());
    }

    #[test]
    fn lex_token_neg_double()
    {
        let first_token = String::from("{");
        let second_token = -123.45;
        let third_token = String::from("}");
        let input = String::from("{ -123.45 }");
        let mut lexer = Lexer::new_copy(&input);

        let mut token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &first_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::DOUBLE);
            assert!(token.is_double());
            assert_eq!(token.as_double().unwrap(), second_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &third_token);
        }

        token_result = lexer.next_token();
        assert!(token_result.is_err());
    }

    #[test]
    fn lex_token_explicit_pos_double_fails()
    {
        let first_token = String::from("{");
        let input = String::from("{ +123.45 }");
        let mut lexer = Lexer::new_copy(&input);

        let mut token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &first_token);
        }

        token_result = lexer.next_token();
        assert!(token_result.is_err());
    }

    #[test]
    fn lex_token_double_neg_double_fails()
    {
        let input = String::from("--123.45");
        let mut lexer = Lexer::new_copy(&input);
        let token_result = lexer.next_token();
        assert!(token_result.is_err());
    }

    #[test]
    fn lex_token_double_big_e_10()
    {
        let first_token = String::from("{");
        let second_token = 123.45E10;
        let third_token = String::from("}");
        let input = String::from("{ 123.45E10 }");
        let mut lexer = Lexer::new_copy(&input);

        let mut token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &first_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::DOUBLE);
            assert!(token.is_double());
            assert_eq!(token.as_double().unwrap(), second_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &third_token);
        }

        token_result = lexer.next_token();
        assert!(token_result.is_err());
    }

    #[test]
    fn lex_token_neg_double_big_e_10()
    {
        let first_token = String::from("{");
        let second_token = -123.45E10;
        let third_token = String::from("}");
        let input = String::from("{ -123.45E10 }");
        let mut lexer = Lexer::new_copy(&input);

        let mut token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &first_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::DOUBLE);
            assert!(token.is_double());
            assert_eq!(token.as_double().unwrap(), second_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &third_token);
        }

        token_result = lexer.next_token();
        assert!(token_result.is_err());
    }

    #[test]
    fn lex_token_neg_zero_zero_one_double()
    {
        let first_token = String::from("{");
        let second_token = -0.01;
        let third_token = String::from("}");
        let input = String::from("{ -0.01 }");
        let mut lexer = Lexer::new_copy(&input);

        let mut token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &first_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::DOUBLE);
            assert!(token.is_double());
            assert_eq!(token.as_double().unwrap(), second_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &third_token);
        }

        token_result = lexer.next_token();
        assert!(token_result.is_err());
    }

    #[test]
    fn lex_token_one_e_ten_double()
    {
        let first_token = String::from("{");
        let second_token = 1.0e10;
        let third_token = String::from("}");
        let input = String::from("{ 1.0e10 }");
        let mut lexer = Lexer::new_copy(&input);

        let mut token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &first_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::DOUBLE);
            assert!(token.is_double());
            assert_eq!(token.as_double().unwrap(), second_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &third_token);
        }

        token_result = lexer.next_token();
        assert!(token_result.is_err());
    }

    #[test]
    fn lex_token_one_e_negative_ten_double()
    {
        let first_token = String::from("{");
        let second_token = 1.5e-10;
        let third_token = String::from("}");
        let input = String::from("{ 1.5e-10 }");
        let mut lexer = Lexer::new_copy(&input);

        let mut token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &first_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::DOUBLE);
            assert!(token.is_double());
            assert_eq!(token.as_double().unwrap(), second_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &third_token);
        }

        token_result = lexer.next_token();
        assert!(token_result.is_err());
    }

    #[test]
    fn lex_token_one_e_ten_point_five_double_fails()
    {
        let first_token = String::from("{");
        let input = String::from("{ 1.0e10.5 }");
        let mut lexer = Lexer::new_copy(&input);

        let mut token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &first_token);
        }

        token_result = lexer.next_token();
        assert!(token_result.is_err());
    }

    #[test]
    fn lex_token_one_e_neg_neg_ten_double_fails()
    {
        let first_token = String::from("{");
        let input = String::from("{ 1.0e--10 }");
        let mut lexer = Lexer::new_copy(&input);

        let mut token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &first_token);
        }

        token_result = lexer.next_token();
        assert!(token_result.is_err());
    }

    #[test]
    fn lex_token_neg_dot_neg_double_fails()
    {
        let first_token = String::from("{");
        let input = String::from("{ -.- }");
        let mut lexer = Lexer::new_copy(&input);

        let mut token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &first_token);
        }

        token_result = lexer.next_token();
        assert!(token_result.is_err());
    }

    #[test]
    fn lex_token_neg_five_dot_neg_five_double_fails()
    {
        let first_token = String::from("{");
        let input = String::from("{ -5.-5 }");
        let mut lexer = Lexer::new_copy(&input);

        let mut token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &first_token);
        }

        token_result = lexer.next_token();
        assert!(token_result.is_err());
    }

    #[test]
    fn lex_token_neg_only_double_fails()
    {
        let first_token = String::from("{");
        let input = String::from("{ - }");
        let mut lexer = Lexer::new_copy(&input);

        let mut token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &first_token);
        }

        token_result = lexer.next_token();
        assert!(token_result.is_err());
    }

    #[test]
    fn lex_token_double_leading_dot()
    {
        let first_token = String::from("{");
        let second_token = 0.45;
        let third_token = String::from("}");
        let input = String::from("{ .45 }");
        let mut lexer = Lexer::new_copy(&input);

        let mut token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &first_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::DOUBLE);
            assert!(token.is_double());
            assert_eq!(token.as_double().unwrap(), second_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &third_token);
        }

        token_result = lexer.next_token();
        assert!(token_result.is_err());
    }

    #[test]
    fn lex_token_double_leading_negative()
    {
        let first_token = String::from("{");
        let second_token = -123.45;
        let third_token = String::from("}");
        let input = String::from("{ -123.45 }");
        let mut lexer = Lexer::new_copy(&input);

        let mut token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &first_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::DOUBLE);
            assert!(token.is_double());
            assert_eq!(token.as_double().unwrap(), second_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &third_token);
        }

        token_result = lexer.next_token();
        assert!(token_result.is_err());
    }

    #[test]
    fn lex_token_null()
    {
        let first_token = String::from("{");
        let third_token = String::from("}");
        let input = String::from("{ null }");
        let mut lexer = Lexer::new_move(input);

        let mut token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &first_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::NULL);
            assert!(token.is_null());
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &third_token);
        }

        token_result = lexer.next_token();
        assert!(token_result.is_err());
    }

    #[test]
    fn lex_string()
    {
        let first_token = String::from("{");
        let second_token = String::from("Hi");
        let third_token = String::from("}");
        let input = String::from("{ \"Hi\" }");
        let mut lexer = Lexer::new_move(input);

        let mut token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &first_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::STRING);
            assert!(token.is_string());
            assert_eq!(token.as_string().unwrap(), &second_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &third_token);
        }

        token_result = lexer.next_token();
        assert!(token_result.is_err());
    }

    #[test]
    fn lex_escape_characters()
    {
        let first_token = String::from("{");
        let second_token = String::from("\"Hi\"");
        let third_token = String::from("}");
        let input = String::from("{ \"\\\"Hi\\\"\" }");
        let mut lexer = Lexer::new_copy(&input);

        let mut token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &first_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::STRING);
            assert!(token.is_string());
            assert_eq!(token.as_string().unwrap(), &second_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &third_token);
        }

        token_result = lexer.next_token();
        assert!(token_result.is_err());
    }

    #[test]
    fn lex_escape_newline()
    {
        let first_token = '\n';
        let input = String::from("\\n");
        let mut lexer = Lexer::new_copy(&input);

        let mut token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::CHAR);
            assert!(token.is_char());
            assert_eq!(token.as_char().unwrap(), first_token);
        }

        token_result = lexer.next_token();
        assert!(token_result.is_err());
    }

    #[test]
    fn lex_escape_carriage_return()
    {
        let first_token = '\r';
        let input = String::from("\\r");
        let mut lexer = Lexer::new_copy(&input);

        let mut token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::CHAR);
            assert!(token.is_char());
            assert_eq!(token.as_char().unwrap(), first_token);
        }

        token_result = lexer.next_token();
        assert!(token_result.is_err());
    }

    #[test]
    fn lex_escape_single_quote()
    {
        let first_token = '\'';
        let input = String::from("\\'");
        let mut lexer = Lexer::new_copy(&input);

        let mut token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::CHAR);
            assert!(token.is_char());
            assert_eq!(token.as_char().unwrap(), first_token);
        }

        token_result = lexer.next_token();
        assert!(token_result.is_err());
    }

    #[test]
    fn lex_escape_double_quote()
    {
        let first_token = '"';
        let input = String::from("\\\"");
        let mut lexer = Lexer::new_copy(&input);

        let mut token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::CHAR);
            assert!(token.is_char());
            assert_eq!(token.as_char().unwrap(), first_token);
        }

        token_result = lexer.next_token();
        assert!(token_result.is_err());
    }

    #[test]
    fn lex_simple_key_value_pair()
    {
        let first_token = String::from("{");
        let second_token = String::from("key");
        let third_token = String::from(":");
        let fourth_token = String::from("value");
        let fifth_token = String::from("}");
        let input = String::from("{ \"key\": \"value\" }");
        let mut lexer = Lexer::new_copy(&input);

        let mut token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &first_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::STRING);
            assert!(token.is_string());
            assert_eq!(token.as_string().unwrap(), &second_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &third_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::STRING);
            assert!(token.is_string());
            assert_eq!(token.as_string().unwrap(), &fourth_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &fifth_token);
        }

        token_result = lexer.next_token();
        assert!(token_result.is_err());
    }

    #[test]
    fn lex_multi_field_object()
    {
        let first_token = String::from("{");
        let second_token = String::from("1");
        let third_token = String::from(":");
        let fourth_token: f64 = 123.0;
        let fifth_token = String::from(",");
        let sixth_token = String::from("2");
        let seventh_token = &third_token;
        let eighth_token: f64 = 456.0;
        let ninth_token = String::from("}");
        let input = String::from("{ \"1\": 123, \"2\": 456 }");
        let mut lexer = Lexer::new_copy(&input);

        let mut token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &first_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::STRING);
            assert!(token.is_string());
            assert_eq!(token.as_string().unwrap(), &second_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &third_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::DOUBLE);
            assert!(token.is_double());
            assert_eq!(token.as_double().unwrap(), fourth_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &fifth_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::STRING);
            assert!(token.is_string());
            assert_eq!(token.as_string().unwrap(), &sixth_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), seventh_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::DOUBLE);
            assert!(token.is_double());
            assert_eq!(token.as_double().unwrap(), eighth_token);
        }

        token_result = lexer.next_token();

        {
            let token = token_result.unwrap();
            assert_eq!(token.get_type(), EnumTokenType::SYMBOL);
            assert!(token.is_symbol());
            assert_eq!(token.as_symbol().unwrap(), &ninth_token);
        }

        token_result = lexer.next_token();
        assert!(token_result.is_err());
    }
}


#[allow(unused_imports)]
use crate::parser::token::{TokenTrait, TokenBool, TokenDouble, TokenSymbol};
use crate::utils::string_utils::StringBuilder;

use std::collections::hash_map::HashMap;
use std::rc::Rc;

pub struct Lexer
{
    input: String,
    index: usize,
    lookup_table: HashMap<char, fn(&mut Lexer, char) -> Result<Rc<dyn TokenTrait>, String>>,
    buffer: StringBuilder,
}

impl Lexer
{
    pub fn new(input: &String) -> Self
    {
        let mut result = Self { input: input.clone(), index: 0, lookup_table: HashMap::new(), buffer: StringBuilder::new(4096) };
        result.init_table();

        return result;
    }

    fn init_table(&mut self)
    {
        self.lookup_table.insert('.', handle_number);
        self.lookup_table.insert('{', handle_symbol);
        self.lookup_table.insert('}', handle_symbol);

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
            self.lookup_table.insert(char::from_digit(i, 10).expect("Failed to conver i to a char"), handle_number);
        }
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

fn handle_number(inst: &mut Lexer, ch: char) -> Result<Rc<dyn TokenTrait>, String>
{
    inst.buffer.append_char(ch);
    let mut seen_dot = false;

    loop
    {
        let cur_char_opt = inst.next_char();

        match cur_char_opt
        {
            Some(cur_char) =>
            {
                if cur_char >= '0' && cur_char <= '9'
                {
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

    let num = output.parse::<f64>().expect("Failed to parse output");
    return Ok(Rc::new(TokenDouble::new(num)));
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
                if !cur_char.is_ascii_whitespace()
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

    return Ok(Rc::new(TokenSymbol::new(output)));
}

